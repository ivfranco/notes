use curl::easy::Easy;
use web_proxy::{
    Result,
    server::run_server,
};
use std::thread;

#[test]
fn http_get_test() -> Result<()> {
    thread::spawn(|| {
        run_server(4444).unwrap();
    });

    let mut client = Easy::new();
    client.url("http://milk.com").unwrap();
    client.proxy("localhost:4444").unwrap();
    let mut buf: Vec<u8> = vec![];

    {
        let mut transfer = client.transfer();
        transfer.write_function(|chunk| {
            buf.extend_from_slice(chunk);
            Ok(chunk.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    println!("{}", std::str::from_utf8(&buf).unwrap());
    // the page has length ~3300 bytes
    // unlikely to change, it's around there since 1994
    assert!(buf.len() > 3000);

    Ok(())
}