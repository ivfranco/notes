use crate::Checksum;

pub struct Fletcher16 {
    c0: u16,
    c1: u16,
}

impl Fletcher16 {
    pub fn new() -> Self {
        Self { c0: 0, c1: 0 }
    }
}

impl Default for Fletcher16 {
    fn default() -> Self {
        Self::new()
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
        [(self.c0 & 0xff) as u8, (self.c1 & 0xff) as u8]
    }

    fn clear(&mut self) {
        *self = Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fletcher16() {
        assert_eq!(Fletcher16::new().digest(&[0x01, 0x02]), [0x03, 0x04]);
    }
}
