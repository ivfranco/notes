struct RudpServer {
    acknowledged: u32,
}

struct Stub {
    buf: Vec<u8>,
    remaining: u32,
}
