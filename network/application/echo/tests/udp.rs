use assert_cmd::prelude::*;
use std::{io, process::Command, thread, time::Duration};

#[test]
fn udp_test() -> io::Result<()> {
    let client = "4444";
    let server = "5555";

    Command::cargo_bin("server")
        .unwrap()
        .arg(client)
        .arg(server)
        .spawn()?;

    // ensure server is fully set up
    thread::sleep(Duration::from_millis(200));

    Command::cargo_bin("client")
        .unwrap()
        .arg(client)
        .arg(server)
        .with_stdin()
        .buffer("abab")
        .assert()
        .stdout("ABAB")
        .success();

    Ok(())
}
