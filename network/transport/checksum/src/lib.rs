use std::mem::size_of;

/// Internet checksum defined in RFC1071 for up to 2^16 bytes.
pub fn checksum(bytes: &[u8]) -> [u8; 2] {
    assert!(bytes.len() < 1 << ((size_of::<u32>() - size_of::<u16>()) * 8));

    let mut sum = bytes.chunks(size_of::<u16>()).fold(0u32, |mut sum, chunk| {
        sum += u32::from(chunk[0]) << 8;
        sum += u32::from(chunk.get(1).copied().unwrap_or(0));
        sum
    });

    const HIGH_MASK: u32 = 0xff_ff_00_00;

    while sum & HIGH_MASK != 0 {
        let high = (sum & HIGH_MASK) >> (size_of::<u16>() * 8);
        sum &= !HIGH_MASK;
        sum += high;
    }

    (!sum as u16).to_be_bytes()
}

#[test]
fn checksum_test() {
    let bytes = &[0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
    assert_eq!(checksum(bytes), [!0xdd, !0xf2]);
}

#[test]
fn reference_test() {
    assert_eq!(checksum(b"Networking"), internet_checksum::checksum(b"Networking"));
}