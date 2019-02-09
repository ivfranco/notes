use std::collections::HashMap;

enum Tag {
    Keyword(u32),
    Id,
    Int(u32),
    Float(f64),
    Operator,
    Comment,
}

mod tag {
    pub const TRUE: u32 = 0;
    pub const FALSE: u32 = 1;
}

struct Token {
    tag: Tag,
    lexeme: String,
}

impl Token {
    fn new_num(lexeme: String) -> Option<Self> {
        let tag = if let Ok(int) = lexeme.parse::<u32>() {
            Tag::Int(int)
        } else if let Ok(float) = lexeme.parse::<f64>() {
            Tag::Float(float)
        } else {
            return None;
        };

        Some(Token { tag, lexeme })
    }

    fn new_word(lexeme: String, reserved: &HashMap<&'static str, u32>) -> Self {
        let tag = if let Some(tag) = reserved.get(lexeme.as_str()) {
            Tag::Keyword(*tag)
        } else {
            Tag::Id
        };

        Token { tag, lexeme }
    }
}

struct Lexer<'a> {
    buf: &'a [u8],
    pos: usize,
    reserved: HashMap<&'static str, u32>,
}

enum LexerError {
    UnexpectedEof,
    UnexpectedByte(usize),
    MalformedToken(usize),
}

impl<'a> Lexer<'a> {
    fn new(buf: &'a [u8]) -> Self {
        let reserved = [("true", tag::TRUE), ("false", tag::FALSE)]
            .iter()
            .cloned()
            .collect();

        Lexer {
            buf,
            pos: 0,
            reserved,
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.buf.len()
    }

    fn peek_u8(&self, offset: usize) -> Option<u8> {
        let pos = self.pos + offset;
        self.buf.get(pos).cloned()
    }

    fn read_u8(&mut self) -> Result<u8, LexerError> {
        self.peek_u8(0).ok_or(LexerError::UnexpectedEof)
    }

    fn consume_u8(&mut self, byte: u8) -> Result<(), LexerError> {
        let pos = self.pos;
        let next = self.read_u8()?;
        if byte == next {
            Ok(())
        } else {
            Err(LexerError::UnexpectedByte(pos))
        }
    }

    fn read_while<F>(&mut self, f: F) -> String
    where
        F: Fn(u8) -> bool,
    {
        if self.eof() {
            String::new()
        } else {
            let lexeme: String = (&self.buf[self.pos..])
                .iter()
                .take_while(|b| f(**b))
                .cloned()
                .map(char::from)
                .collect();

            self.pos += lexeme.len();
            lexeme
        }
    }

    fn skip_whitespaces(&mut self) {
        self.read_while(|b| b.is_ascii_whitespace());
    }

    fn read_num(&mut self) -> Result<Token, LexerError> {
        let pos = self.pos;
        let lexeme = self.read_while(|b| b.is_ascii_digit() || b == b'.');
        Token::new_num(lexeme).ok_or_else(|| LexerError::MalformedToken(pos))
    }

    fn read_word(&mut self) -> Result<Token, LexerError> {
        let lexeme = self.read_while(|b| b.is_ascii_alphanumeric());
        Ok(Token::new_word(lexeme, &self.reserved))
    }

    fn read_comment(&mut self) -> Result<Token, LexerError> {
        self.consume_u8(b'/')?;
        let pos = self.pos;
        let lexeme = match self.read_u8()? {
            b'/' => {
                let lexeme = self.read_while(|b| b != b'\n');
                self.consume_u8(b'\n')?;
                lexeme
            }
            b'*' => {
                if let Some(offset) = (&self.buf[self.pos..])
                    .windows(2)
                    .position(|word| word == b"*/")
                {
                    let lexeme = &self.buf[self.pos..self.pos + offset];
                    String::from_utf8(lexeme.to_vec()).expect("Error: Non utf8 string")
                } else {
                    return Err(LexerError::UnexpectedEof);
                }
            }
            _ => return Err(LexerError::UnexpectedByte(pos)),
        };

        Ok(Token {
            tag: Tag::Comment,
            lexeme,
        })
    }
}
