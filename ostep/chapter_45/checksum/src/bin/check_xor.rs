use checksum::main_common;
use checksum::xor::Xor;

fn main() {
    let xor = Xor::new();
    main_common(xor);
}
