use std::{net::Ipv4Addr, sync::Arc, thread};

use rand::{distributions::Standard, Rng, SeedableRng};
use rudp::{client::RudpClient, server::RudpServer};

#[test]
fn fragmented() {
    const BYTES_LEN: usize = 10 * 1024;

    let rng = rand::rngs::StdRng::from_entropy();
    let bytes = Arc::new(
        rng.sample_iter(Standard)
            .take(BYTES_LEN)
            .collect::<Vec<u8>>(),
    );

    let mut server = RudpServer::bind((Ipv4Addr::LOCALHOST, 8080)).unwrap();
    let handle = {
        let bytes = Arc::clone(&bytes);
        thread::spawn(move || {
            let mut buf = [0u8; BYTES_LEN];
            server.recv(&mut buf).unwrap();
            assert_eq!(buf, bytes.as_slice());
        })
    };

    let client = RudpClient::connect((Ipv4Addr::LOCALHOST, 8080)).unwrap();
    client.send(&bytes).unwrap();

    handle.join().unwrap();
}
