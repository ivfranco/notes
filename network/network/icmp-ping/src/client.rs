use crate::{
    epoch,
    protocol::{consts::*, *},
};
use log::{info, warn};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{
    io::{Error, ErrorKind, Result},
    net::{Ipv4Addr, SocketAddr},
    thread,
    time::{Duration, Instant},
};

pub fn ping(ip: &str) -> Result<()> {
    let addr = ip
        .parse::<Ipv4Addr>()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?;
    let socket_addr = SocketAddr::from((addr, 80 /* won't be used */));
    let client = PingClient::new()?;
    let results = client.ping(&SockAddr::from(socket_addr))?;

    println!("Ping statistics for {}:", ip);
    let received = received(&results);
    println!(
        "    Packets: Sent = {}, Received = {}, Lost = {} ({:.2}% loss)",
        results.len(),
        received,
        results.len() - received,
        ((results.len() - received) as f64 / results.len() as f64) * 100.0,
    );
    if let Some((min, max, avg)) = minmaxavg(&results) {
        println!("Approximate round trip times in milli-seconds:");
        println!("    Minimum = {}ms, Maximum = {}ms, Average = {}ms", min, max, avg);
    }
    Ok(())
}

fn received(results: &[Option<Duration>]) -> usize {
    results.iter().filter(|rtt| rtt.is_some()).count()
}

fn minmaxavg(results: &[Option<Duration>]) -> Option<(u128, u128, u128)> {
    let min = results.iter().filter_map(|rtt| *rtt).min()?.as_millis();
    let max = results.iter().filter_map(|rtt| *rtt).max()?.as_millis();
    let avg = (results.iter().filter_map(|rtt| *rtt).sum::<Duration>() / results.len() as u32).as_millis();
    Some((min, max, avg))
}

// should be big enough for echo reply
const BUF_LEN: usize = 0x100;
const IP_HEADER_LEN: usize = 20;
const TIMEOUT: Duration = Duration::from_secs(3);
const PAUSE: Duration = Duration::from_secs(1);

struct PingClient {
    socket: Socket,
}

impl PingClient {
    fn new() -> Result<Self> {
        let socket = Socket::new(Domain::ipv4(), Type::raw(), Some(Protocol::icmpv4()))?;
        socket.set_read_timeout(Some(TIMEOUT))?;
        Ok(Self { socket })
    }

    fn ping(&self, addr: &SockAddr) -> Result<Vec<Option<Duration>>> {
        let ident = epoch() as u16;
        let mut results = vec![];

        println!(
            "Pinging {:?} with {} bytes of data",
            addr.as_inet().unwrap().ip(),
            DATA_LEN
        );

        for seq in 0..10 {
            self.send(ident, seq, addr)?;

            let rtt = self.recv(ident, seq)?;
            if let Some(duration) = rtt {
                println!(
                    "Received reply from {:?}: bytes={}, time={}ms",
                    addr.as_inet().unwrap().ip(),
                    DATA_LEN,
                    duration.as_millis()
                );

                if duration < PAUSE {
                    thread::sleep(PAUSE - duration);
                }
            } else {
                println!("Request timed out");
            }

            results.push(rtt);
        }

        Ok(results)
    }

    fn send(&self, ident: u16, seq: u16, addr: &SockAddr) -> Result<()> {
        let echo = ICMPEcho::new(ident, seq);
        self.socket.send_to(&echo, addr)?;
        info!("Sent echo packet with ident={}, seq={}", ident, seq);
        Ok(())
    }

    fn recv(&self, ident: u16, seq: u16) -> Result<Option<Duration>> {
        let mut buf = [0; BUF_LEN];
        let now = Instant::now();
        loop {
            match self.socket.recv(&mut buf) {
                Ok(len) => {
                    let reply = if let Some(reply) = ICMPEcho::from_reply(&buf[IP_HEADER_LEN..len])
                    {
                        reply
                    } else {
                        warn!("Received malformed reply");
                        continue;
                    };

                    if reply.ident() != ident || reply.seq() != seq {
                        warn!(
                            "Out of order reply, expecting ident={}, seq={}, received ident={}, seq={}", 
                            ident, seq, reply.ident(), reply.seq()
                        );
                        continue;
                    }

                    return Ok(Some(now.elapsed()));
                }
                Err(ref err) if err.kind() == ErrorKind::TimedOut => {
                    return Ok(None);
                }
                Err(err) => return Err(err),
            }
        }
    }
}
