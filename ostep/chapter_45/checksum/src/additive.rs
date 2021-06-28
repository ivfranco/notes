use crate::Checksum;

pub struct Add(u8);

impl Add {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for Add {
    fn default() -> Self {
        Self::new()
    }
}

impl Checksum<1> for Add {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = bytes
            .iter()
            .fold(self.0, |sum, byte| sum.wrapping_add(*byte));
    }

    fn finish(&self) -> [u8; 1] {
        [self.0]
    }

    fn clear(&mut self) {
        self.0 = 0;
    }
}
