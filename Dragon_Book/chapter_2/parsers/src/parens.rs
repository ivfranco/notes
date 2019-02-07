use std::fmt;

enum Parens {
    Empty,
    Pairs(Box<Parens>, Box<Parens>),
}

impl fmt::Debug for Parens {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Parens::Pairs(lhs, rhs) => {
                write!(f, "({:?}){:?}", lhs, rhs)?;
                Ok(())
            }
            Parens::Empty => Ok(()),
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

    fn parens(&mut self) -> Parens {
        match self.peek() {
            Some(b'(') => {
                self.consume(b'(');
                let lhs = self.parens();
                self.consume(b')');
                let rhs = self.parens();
                Parens::Pairs(Box::new(lhs), Box::new(rhs))
            }
            Some(b')') | None => Parens::Empty,
            Some(byte) => panic!("Unexpected token: {}", byte),
        }
    }

    fn eof(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

#[test]
fn parens_test() {
    let strings = [
        "(()())((()()))()",
        "(())",
        "((()))()(()()())",
        "()(((()))())",
        "()()",
        "()",
        "((()(()())))",
        "(()(((()))))",
        "()((()))",
        "((()))",
        "",
    ];

    for string in strings.iter() {
        let mut parser = Parser::new(string.as_bytes());
        let parens = parser.parens();
        assert!(parser.eof(), "Unexhaused input: {}", string);
        assert_eq!(&format!("{:?}", parens), string);
    }
}
