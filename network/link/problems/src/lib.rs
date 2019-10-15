pub use checksum::checksum;

pub fn short_crc(mut data: u64, gen: u64, rem: u32) -> u64 {
    data <<= rem;

    let mut divisor = gen << (gen.leading_zeros() - data.leading_zeros());
    while divisor >= gen {
        if data.leading_zeros() <= divisor.leading_zeros() {
            data ^= divisor;
        }
        divisor >>= 1;
    }

    data
}

#[test]
fn crc_test() {
    assert_eq!(short_crc(0b10_1110, 0b1001, 3), 0b011);
}