use crate::Checksum;

/// Thanks https://wiki.plasticheart.info/algorithm-crc16
pub struct Crc16 {
    poly: u16,
    init: u16,
    sum: u16,
    refin: bool,
    refout: bool,
    xorout: u16,
}

impl Crc16 {
    pub fn new(poly: u16, mut init: u16, refin: bool, refout: bool, xorout: u16) -> Self {
        if refin {
            init = init.reverse_bits();
        }

        Self {
            poly,
            init,
            sum: init,
            refin,
            refout,
            xorout,
        }
    }

    pub fn ccitt_false() -> Self {
        Self::new(0x1021, 0xffff, false, false, 0x0000)
    }
}

impl Checksum<2> for Crc16 {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.sum ^= u16::from(if self.refin {
                byte.reverse_bits()
            } else {
                byte
            }) << 8;

            for _ in 0..8 {
                self.sum = if self.sum & 0x8000 == 0 {
                    self.sum << 1
                } else {
                    (self.sum << 1) ^ self.poly
                }
            }
        }
    }

    fn finish(&self) -> [u8; 2] {
        let x = if self.refout {
            self.sum.reverse_bits()
        } else {
            self.sum
        };

        (x ^ self.xorout).to_be_bytes()
    }

    fn clear(&mut self) {
        self.sum = self.init;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc16_ccitt_false() {
        assert_eq!(Crc16::ccitt_false().digest(b"123456789"), [0x29, 0xb1]);
        assert_eq!(
            Crc16::ccitt_false().digest(&[0x12, 0x34, 0x56, 0x78, 0x9]),
            [0x4b, 0x7a]
        );
    }
}
