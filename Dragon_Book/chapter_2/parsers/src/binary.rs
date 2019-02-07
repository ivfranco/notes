use std::fmt;

enum Binary {
    Nest(Box<Binary>),
    Base,
}

impl fmt::Debug for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Binary::Nest(inner) => write!(f, "0{:?}1", inner),
            Binary::Base => write!(f, "01"),
        }
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Parser { bytes, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).cloned()
    }

    fn peekn(&self, n: usize) -> Option<Vec<u8>> {
        if self.pos + n <= self.bytes.len() {
            Some(self.bytes[self.pos..self.pos + n].to_owned())
        } else {
            None
        }
    }

    fn read(&mut self) -> u8 {
        if let Some(byte) = self.peek() {
            self.pos += 1;
            byte
        } else {
            panic!("Error: Unexpected EOF");
        }
    }

    fn consume(&mut self, c: u8) {
        assert_eq!(self.read(), c);
    }

    fn eof(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn binary(&mut self) -> Binary {
        match self.peekn(2) {
            Some(ref vec) if vec == &[b'0', b'0'] => {
                self.consume(b'0');
                let inner = self.binary();
                self.consume(b'1');
                Binary::Nest(Box::new(inner))
            }
            Some(ref vec) if vec == &[b'0', b'1'] => {
                self.consume(b'0');
                self.consume(b'1');
                Binary::Base
            }
            Some(vec) => panic!("Unexpected tokens: {:?}", vec),
            None => panic!("Unexpected EOF"),
        }
    }
}

#[test]
fn binary_test() {
    let strings = "01
000000111111
00001111
000111
0000011111
0000000011111111
00000001111111
0011
0000011111
01
0011
0000000011111111
00000001111111
000000111111
00001111
000111";

    for string in strings.lines() {
        let mut parser = Parser::new(string.as_bytes());
        let binary = parser.binary();
        assert!(parser.eof(), "Unexhaused input: {}", string);
        assert_eq!(&format!("{:?}", binary), string);
    }
}
