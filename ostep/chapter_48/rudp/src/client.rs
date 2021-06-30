use std::{
    collections::VecDeque,
    io,
    net::{Ipv4Addr, ToSocketAddrs, UdpSocket},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::Packet;

const CLIENT_TIME_OUT: Duration = Duration::from_secs(5);

struct RawRudpClient {
    /// Unique identifer of a packet.
    sequential_number: u32,
    /// Starts from 1, 0 is reserved for not fragmented packets.
    fragment_identifer: u32,
    /// Sequential number up to but not including `acknowledged` is known as acknowledged by the server.
    acknowledged: u32,
    /// Packet with their deadline naturally sorted by their sequential numbers.
    packet_queue: VecDeque<(Packet, Instant)>,
    /// A UDP socket readily connected to the server.
    udp_socket: UdpSocket,
}

impl RawRudpClient {
    fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let udp_socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        udp_socket.connect(addr)?;

        Ok(Self {
            sequential_number: 0,
            fragment_identifer: 1,
            acknowledged: 0,
            packet_queue: VecDeque::new(),
            udp_socket,
        })
    }

    fn send(&mut self, input: &[u8]) -> io::Result<()> {
        let deadline = Instant::now() + CLIENT_TIME_OUT;
        let pushed = if input.len() <= Packet::MAX_SAFE_SIZE {
            let packet = Packet::new(input, self.sequential_number);
            self.packet_queue.push_front((packet, deadline));
            1
        } else {
            // fragment the oversized input
            let mut pushed = 0;
            for packet in Packet::fragment(input, self.sequential_number, self.fragment_identifer) {
                self.packet_queue.push_front((packet, deadline));
                pushed += 1;
            }
            self.fragment_identifer += 1;
            pushed
        };

        self.sequential_number += pushed;

        let mut buf = [0u8; Packet::HEADER_SIZE + Packet::MAX_SAFE_SIZE];
        for (packet, _) in self.packet_queue.range(..pushed as usize).rev() {
            packet.write_to(&mut buf)?;
            self.udp_socket.send(&buf)?;
        }

        Ok(())
    }

    fn on_ack(&mut self, ack: u32) {
        let mut dropped = 0;
        for (packet, _) in self.packet_queue.iter() {
            if packet.sequential_number <= ack {
                dropped += 1;
            } else {
                break;
            }
        }

        self.packet_queue.drain(..dropped);
        self.acknowledged = self.acknowledged.max(ack + 1);
    }

    fn on_timeout(&mut self) -> io::Result<()> {
        let now = Instant::now();
        let mut buf = [0u8; Packet::HEADER_SIZE + Packet::MAX_SAFE_SIZE];
        for (packet, deadline) in self.packet_queue.iter() {
            if *deadline <= now {
                packet.write_to(&mut buf)?;
                self.udp_socket.send(&buf)?;
            } else {
                break;
            }
        }

        Ok(())
    }
}

pub struct RudpClient {
    /// TODO: fells wrong, the socket should not be in the mutex
    inner: Arc<Mutex<RawRudpClient>>,
}

impl RudpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let raw = RawRudpClient::connect(addr)?;
        Ok(Self {
            inner: Arc::new(Mutex::new(raw)),
        })
    }

    pub fn send(&self, input: &[u8]) -> io::Result<()> {
        let mut lock = self.inner.lock().unwrap();
        lock.send(input)
    }
}
