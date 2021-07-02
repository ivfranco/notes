use std::{net::Ipv4Addr, thread, time::Duration};

use rudp::{client::RudpClient, server::RudpServer};

#[test]
fn connection() {
    let mut server = RudpServer::bind(8080).unwrap();
    thread::spawn(move || {
        server.listen().unwrap();
    });

    let client = RudpClient::connect((Ipv4Addr::LOCALHOST, 8080)).unwrap();
    for i in 0u32..10 {
        client.send(&i.to_be_bytes()).unwrap();
    }

    thread::sleep(Duration::from_secs(10));
}
