use std::fmt;

enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    A,
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Expr::Add(lhs, rhs) => write!(f, "+{:?}{:?}", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "-{:?}{:?}", lhs, rhs),
            Expr::A => write!(f, "a"),
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

    fn expr(&mut self) -> Expr {
        match self.read() {
            b'+' => {
                let lhs = self.expr();
                let rhs = self.expr();
                Expr::Add(Box::new(lhs), Box::new(rhs))
            }
            b'-' => {
                let lhs = self.expr();
                let rhs = self.expr();
                Expr::Sub(Box::new(lhs), Box::new(rhs))
            }
            b'a' => Expr::A,
            _ => panic!("Error: Unexpected token, non of '+', '-' or 'a'"),
        }
    }

    fn eof(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

#[test]
fn prefix_test() {
    let strings = [
        "--aaa",
        "+-a-aaa",
        "+a-aa",
        "+a++aaa",
        "a",
        "-+-+aa+a+aaaa",
        "-+aa-+aa--aaa",
        "++aaa",
        "-a-a+aa",
        "-a+--aaa-a-aa",
    ];

    for string in strings.iter() {
        let mut parser = Parser::new(string.as_bytes());
        let expr = parser.expr();
        assert!(parser.eof(), "Unexhausted input: {}", string);
        assert_eq!(&format!("{:?}", expr), string);
    }
}
