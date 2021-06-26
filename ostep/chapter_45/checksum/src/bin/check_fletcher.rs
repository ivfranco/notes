use checksum::{main_common, Checksum};

fn main() {
    let checksum = Fletcher16::new();
    main_common(checksum);
}

struct Fletcher16 {
    c0: u16,
    c1: u16,
}

impl Fletcher16 {
    fn new() -> Self {
        Self { c0: 0, c1: 0 }
    }
}

impl Checksum<2> for Fletcher16 {
    fn write(&mut self, bytes: &[u8]) {
        let Self { c0, c1 } = self;
        for &byte in bytes {
            *c0 = (*c0 + byte as u16) % 255;
            *c1 = (*c1 + *c0) % 255;
        }
    }

    fn finish(&self) -> [u8; 2] {
        // Output of Fletcher's checksum on Wikipedia is in big endian.
        [(self.c1 & 0xff) as u8, (self.c0 & 0xff) as u8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fletcher16() {
        assert_eq!(Fletcher16::new().digest(&[0x01, 0x02]), [0x04, 0x03]);
    }
}
