use std::ops::Not;

pub fn u16_checksum(bytes: &[u8]) -> [u8; 2] {
    let mut acc = 0u32;
    let mut pair = [0u8; 2];
    let high_mask = 0xff_ff_00_00u32;

    for chunk in bytes.chunks(2) {
        if chunk.len() == 2 {
            pair.copy_from_slice(chunk);
        } else {
            pair[0] = chunk[0];
            pair[1] = 0;
        }

        acc += u32::from(u16::from_be_bytes(pair));
        if acc & high_mask != 0 {
            acc &= !high_mask;
            acc += 1;
        }
    }

    (acc as u16).not().to_be_bytes()
}

#[test]
fn u16_checksum_test() {
    let bytes = &[
        0b0110_0110,
        0b0110_0000,
        0b0101_0101,
        0b0101_0101,
        0b1000_1111,
        0b0000_1100,
    ];
    assert_eq!(
        u16::from_be_bytes(u16_checksum(bytes)),
        0b1011_0101_0011_1101
    );
}
