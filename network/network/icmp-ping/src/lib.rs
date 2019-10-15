pub mod client;
mod protocol;

pub use checksum::checksum;
use std::time::SystemTime;

pub const ZERO_SUM: [u8; 2] = [0, 0];

pub fn epoch() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}