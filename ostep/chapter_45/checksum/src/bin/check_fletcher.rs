use checksum::fletcher::Fletcher16;
use checksum::main_common;

fn main() {
    let checksum = Fletcher16::new();
    main_common(checksum);
}
