use checksum::main_common;
use checksum::Checksum;

fn main() {
    let xor = Xor::new();
    main_common(xor);
}

struct Xor(u8);

impl Xor {
    fn new() -> Self {
        Xor(0)
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
