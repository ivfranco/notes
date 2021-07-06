use std::{
    cmp::{self, Ordering},
    io,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
};

use crate::{AckPacket, DataPacket, MAX_SAFE_DATAGRAM_SIZE};

pub struct RudpServer {
    /// The next sequential number the server is expecting.
    acknowledged: u32,
    udp_socket: UdpSocket,
}

impl RudpServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let udp_socket = UdpSocket::bind(addr)?;
        Ok(Self {
            acknowledged: 0,
            udp_socket,
        })
    }

    fn ack(&mut self, sn: u32, peer: SocketAddr) -> io::Result<()> {
        let mut buf = [0u8; AckPacket::SIZE];
        AckPacket::new(sn).write_to(&mut buf)?;
        self.udp_socket.send_to(&buf, peer)?;

        Ok(())
    }

    fn recv_packet(&mut self) -> io::Result<DataPacket> {
        let mut packet_buf = [0u8; MAX_SAFE_DATAGRAM_SIZE];

        loop {
            let (amt, peer) = self.udp_socket.recv_from(&mut packet_buf)?;
            let packet = DataPacket::read_from(&packet_buf[..amt])?;

            match packet.sequential_number.cmp(&self.acknowledged) {
                Ordering::Equal => {
                    self.acknowledged += 1;
                    self.ack(self.acknowledged, peer)?;
                    return Ok(packet);
                }
                Ordering::Less => {
                    // resend ack for old packet
                    self.ack(self.acknowledged, peer)?;
                }
                Ordering::Greater => {
                    // out-of-order packets are ignored
                }
            }
        }
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut packet = self.recv_packet()?;

        if packet.fragment_identifer == 0 {
            // packet is not fragmented
            let amt = cmp::max(packet.datagram.len(), buf.len());
            (&mut buf[..amt]).copy_from_slice(&packet.datagram[..amt]);
            Ok(amt)
        } else {
            // packet is fragmented
            let mut datagram = vec![0u8; packet.fragment_total as usize];
            let mut remain = packet.fragment_total;

            loop {
                let chunk = &mut datagram[packet.fragment_index as usize
                    ..packet.fragment_index as usize + packet.datagram.len()];
                chunk.copy_from_slice(&packet.datagram);
                remain -= packet.datagram.len() as u32;
                if remain == 0 {
                    break;
                } else {
                    packet = self.recv_packet()?;
                }
            }

            let amt = cmp::max(buf.len(), datagram.len());
            (&mut buf[..amt]).copy_from_slice(&datagram[..amt]);
            Ok(amt)
        }
    }
}
