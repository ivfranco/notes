use checksum::{additive::Add, main_common};

fn main() {
    let checksum = Add::new();
    main_common(checksum);
}
