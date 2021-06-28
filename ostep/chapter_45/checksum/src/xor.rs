use crate::Checksum;

pub struct Xor(u8);

impl Xor {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for Xor {
    fn default() -> Self {
        Self::new()
    }
}

impl Checksum<1> for Xor {
    fn write(&mut self, bytes: &[u8]) {
        let xor = bytes.iter().fold(self.0, |xor, byte| xor ^ byte);
        self.0 = xor;
    }

    fn finish(&self) -> [u8; 1] {
        [self.0]
    }

    fn clear(&mut self) {
        *self = Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor() {
        assert_eq!(Xor::new().digest(&[0; 8]), [0]);
        assert_eq!(Xor::new().digest(&[1; 8]), [0]);
        assert_eq!(Xor::new().digest(&[1; 3]), [1]);
    }
}
