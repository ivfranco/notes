use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
};

use crate::{AckPacket, DataPacket};

pub struct RudpServer {
    acknowledged: u32,
    udp_socket: UdpSocket,
}

impl RudpServer {
    pub fn bind(port: u16) -> io::Result<Self> {
        let udp_socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, port))?;
        Ok(Self {
            acknowledged: 0,
            udp_socket,
        })
    }

    pub fn listen(&mut self) -> io::Result<()> {
        let mut buf = [0u8; DataPacket::HEADER_SIZE + DataPacket::MAX_SAFE_SIZE];
        loop {
            let (amt, peer) = self.udp_socket.recv_from(&mut buf)?;
            let packet = DataPacket::read_from(&buf[..amt])?;
            println!("{:?}", packet);
            if packet.sequential_number <= self.acknowledged {
                self.acknowledged = self.acknowledged.max(packet.sequential_number + 1);
                let ack = AckPacket::new(self.acknowledged);
                let amt = ack.write_to(&mut buf)?;
                self.udp_socket.send_to(&buf[..amt], peer)?;
            }
        }
    }
}
