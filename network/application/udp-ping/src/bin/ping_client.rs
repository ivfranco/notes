use random_fast_rng::{local_rng, Random};
use std::{
    cmp, env,
    io::{self, ErrorKind},
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    process,
    time::{Duration, Instant},
};
use udp_ping::{
    consts::{PACKET_LEN, PAYLOAD_LEN},
    PingError, PingPacket, Type,
};

const TRIALS: u16 = 10;

fn main() -> io::Result<()> {
    let (client, server) = parse_host_or_exit();
    send_ping((Ipv4Addr::LOCALHOST, client).into(), server)
}

fn send_ping(client: SocketAddr, server: SocketAddr) -> io::Result<()> {
    let socket = UdpSocket::bind(client)?;
    socket.connect(server)?;
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    println!("Pinging {} with {} bytes of data", server, PAYLOAD_LEN);

    let mut rng = local_rng();
    let identifier = rng.gen();
    let mut rtts = vec![];

    for seuqence_number in 0..TRIALS {
        let now = Instant::now();

        let request_packet = PingPacket::new(Type::Request, identifier, seuqence_number);
        socket.send(request_packet.packet())?;

        let rtt = match recv_or_timeout(&socket, identifier, seuqence_number) {
            Ok(..) => Some(now.elapsed()),
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => None,
                _ => return Err(err),
            },
        };

        if let Some(duration) = rtt {
            println!(
                "Reply from {}: bytes={}, time={}ms",
                server,
                PAYLOAD_LEN,
                duration.as_millis()
            );
            rtts.push(duration.as_millis());
        } else {
            println!("Request timed out.");
        }
    }

    statistics(server, &rtts);

    Ok(())
}

fn statistics(server: SocketAddr, rtts: &[u128]) {
    println!("Ping statistics for {:?}:", server);
    println!(
        "    Packets: Sent = {}, Received = {}, Lost = {}",
        TRIALS,
        rtts.len(),
        usize::from(TRIALS)- rtts.len()
    );
    println!("Approximate round trip times in milli-seconds:");
    println!(
        "    Minimum = {:?}, Maximum = {:?}, Average = {:?}",
        rtts.iter().min(),
        rtts.iter().max(),
        rtts.iter().sum::<u128>() / cmp::max(1, rtts.len()) as u128,
    );
}

fn recv_or_timeout(socket: &UdpSocket, identifier: u16, sequence_number: u16) -> io::Result<()> {
    loop {
        let mut packet = [0; PACKET_LEN];
        socket.recv(&mut packet)?;
        let reply_packet = PingPacket::from_be_bytes(packet).map_err(|err| err.into_io_error())?;

        if identifier != reply_packet.identifier() {
            return Err(PingError::IdentifierMismatch.into());
        }

        if reply_packet.sequence_number() == sequence_number {
            return Ok(());
        }
        // skip reply with different sequence numbers
        // may be packet from last round lagged for more than one second
    }
}

fn parse_host() -> Option<(u16, SocketAddr)> {
    let mut args = env::args();
    // skip executable name
    args.next();
    let client = args.next().and_then(|arg| arg.parse::<u16>().ok())?;
    let server = args.next().and_then(|arg| arg.parse::<SocketAddr>().ok())?;

    Some((client, server))
}

fn parse_host_or_exit() -> (u16, SocketAddr) {
    parse_host().unwrap_or_else(|| {
        eprintln!("Usage: EXEC CLIENT_PORT SERVER_HOST:SERVER_PORT");
        process::exit(1);
    })
}
