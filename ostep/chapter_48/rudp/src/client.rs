use std::{
    collections::VecDeque,
    io::{self, ErrorKind},
    net::{Ipv4Addr, ToSocketAddrs, UdpSocket},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{AckPacket, DataPacket};

const CLIENT_TIME_OUT: Duration = Duration::from_secs(5);

struct RudpState {
    /// Unique identifer of a packet.
    sequential_number: u32,
    /// Starts from 1, 0 is reserved for not fragmented packets.
    fragment_identifer: u32,
    /// Sequential number up to but not including `acknowledged` is known as acknowledged by the server.
    acknowledged: u32,
    /// Packet with their deadline naturally sorted by their sequential numbers.
    packet_queue: VecDeque<(DataPacket, Instant)>,
}

impl RudpState {
    fn new() -> Self {
        Self {
            sequential_number: 0,
            fragment_identifer: 1,
            acknowledged: 0,
            packet_queue: VecDeque::new(),
        }
    }

    fn on_send(&mut self, input: &[u8]) -> io::Result<u32> {
        let deadline = Instant::now() + CLIENT_TIME_OUT;
        let pushed = if input.len() <= DataPacket::MAX_SAFE_SIZE {
            let packet = DataPacket::new(input, self.sequential_number);
            self.packet_queue.push_front((packet, deadline));
            1
        } else {
            // fragment the oversized input
            let mut pushed = 0;
            for packet in
                DataPacket::fragment(input, self.sequential_number, self.fragment_identifer)
            {
                self.packet_queue.push_front((packet, deadline));
                pushed += 1;
            }
            self.fragment_identifer += 1;
            pushed
        };

        self.sequential_number += pushed;
        Ok(pushed)
    }

    fn on_ack(&mut self, ack: u32) {
        let mut dropped = 0;
        for (packet, _) in self.packet_queue.iter() {
            if packet.sequential_number < ack {
                dropped += 1;
            } else {
                break;
            }
        }

        self.packet_queue.drain(..dropped);
        self.acknowledged = self.acknowledged.max(ack);
    }
}

struct RawRudpClient {
    state: Mutex<RudpState>,
    udp_socket: UdpSocket,
}

impl RawRudpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let udp_socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        udp_socket.connect(addr)?;

        Ok(Self {
            state: Mutex::new(RudpState::new()),
            udp_socket,
        })
    }

    fn send(&self, input: &[u8]) -> io::Result<()> {
        let mut state = self.state.lock().unwrap();
        let pushed = state.on_send(input)?;

        let mut buf = [0u8; DataPacket::HEADER_SIZE + DataPacket::MAX_SAFE_SIZE];
        for (packet, _) in state.packet_queue.range(..pushed as usize).rev() {
            let amt = packet.write_to(&mut buf)?;
            self.udp_socket.send(&buf[..amt])?;
        }

        Ok(())
    }
}

pub struct RudpClient {
    inner: Arc<RawRudpClient>,
}

impl RudpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let inner = Arc::new(RawRudpClient::connect(addr)?);
        {
            let client = Arc::clone(&inner);
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
                rudp_background(&client).unwrap();
            });
        }
        Ok(Self { inner })
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<()> {
        self.inner.send(buf)
    }
}

/// Periodically receive acks from the server and resend the timeout packets. Not an ideal solution
/// but should work.
fn rudp_background(client: &RawRudpClient) -> io::Result<()> {
    let mut buf = [0u8; DataPacket::HEADER_SIZE + DataPacket::MAX_SAFE_SIZE];
    let mut state = client.state.lock().unwrap();
    client.udp_socket.set_nonblocking(true)?;

    loop {
        match client.udp_socket.recv(&mut buf) {
            Ok(amt) => {
                let ack = AckPacket::read_from(&buf[..amt])?;
                state.on_ack(ack.sequential_number);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => return Err(e),
        }
    }

    client.udp_socket.set_nonblocking(false)?;
    let now = Instant::now();

    for (packet, deadline) in state.packet_queue.iter() {
        if *deadline <= now {
            packet.write_to(&mut buf)?;
            client.udp_socket.send(&buf)?;
        } else {
            break;
        }
    }

    Ok(())
}
