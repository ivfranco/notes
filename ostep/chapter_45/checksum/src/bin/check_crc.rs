use checksum::crc::Crc16;
use checksum::main_common;

fn main() {
    let checksum = Crc16::ccitt_false();
    main_common(checksum);
}
