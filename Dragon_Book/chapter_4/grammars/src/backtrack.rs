pub struct Parser<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Parser { buf, pos: 0 }
    }

    fn consume_a(&mut self) -> Option<usize> {
        if self.pos < self.buf.len() && self.buf[self.pos] == b'a' {
            self.pos += 1;
            Some(1)
        } else {
            None
        }
    }

    fn s(&mut self) -> Option<usize> {
        self.asa().or_else(|| self.aa())
    }

    fn asa(&mut self) -> Option<usize> {
        let start = self.pos;
        println!("S -> aSa");

        self.consume_a()
            .and_then(|_| self.s())
            .and_then(|inc| self.consume_a().map(|_| inc + 2))
            .or_else(|| {
                self.pos = start;
                println!("S -> aSa failed at pos {}", start);
                None
            })
    }

    fn aa(&mut self) -> Option<usize> {
        let start = self.pos;
        println!("S -> aa");

        self.consume_a()
            .and(self.consume_a())
            .and(Some(2))
            .or_else(|| {
                self.pos = start;
                println!("S -> aa failed at pos {}", start);
                None
            })
    }
}

pub fn recognize(buf: &[u8]) -> Option<usize> {
    let mut parser = Parser::new(buf);
    parser.s()
}

#[test]
fn recognize_test() {
    assert_eq!(recognize(b"aa"), Some(2));
    assert_eq!(recognize(b"aaaa"), Some(4));
    assert_eq!(recognize(b"aaaaaaaa"), Some(8));
    assert_eq!(recognize(b"aaaaaa"), Some(4));
}
