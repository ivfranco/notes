use std::{
    io,
    net::{Ipv4Addr, UdpSocket},
};
use udp_ping::{consts::PACKET_LEN, parse_port_or_exit, PingPacket, Type};

fn main() -> io::Result<()> {
    let port = parse_port_or_exit();
    recv_ping(port)
}

fn recv_ping(port: u16) -> io::Result<()> {
    let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, port))?;
    loop {
        let mut packet = [0; PACKET_LEN];
        let (_, client) = socket.recv_from(&mut packet)?;
        let request_packet =
            PingPacket::from_be_bytes(packet).map_err(|err| err.into_io_error())?;
        let reply_packet = PingPacket::new(
            Type::Reply,
            request_packet.identifier(),
            request_packet.sequence_number(),
        );
        socket.send_to(&reply_packet.packet(), client)?;
    }
}
