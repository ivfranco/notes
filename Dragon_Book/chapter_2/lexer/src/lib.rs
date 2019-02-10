#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
use std::process;

#[derive(Debug)]
enum Tag {
    Keyword(u32),
    Id,
    Int(u32),
    Float(f64),
    Operator,
    Comment,
}

mod consts {
    pub const TRUE: u32 = 0;
    pub const FALSE: u32 = 1;

    pub const OPERATORS: [&str; 12] = [
        "+", "-", "*", "/", "<", ">", "=", ";", "<=", ">=", "==", "!=",
    ];
}

struct Token {
    tag: Tag,
    lexeme: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{:?}, {:?}>", self.tag, self.lexeme)
    }
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

#[derive(Debug)]
enum LexerError {
    UnexpectedEof,
    UnexpectedByte(usize),
    MalformedToken(usize),
}

impl<'a> Lexer<'a> {
    fn new(buf: &'a [u8]) -> Self {
        let reserved = [("true", consts::TRUE), ("false", consts::FALSE)]
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
        if let Some(byte) = self.peek_u8(0) {
            self.pos += 1;
            Ok(byte)
        } else {
            Err(LexerError::UnexpectedEof)
        }
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
                if !self.eof() {
                    self.consume_u8(b'\n')?;
                }
                lexeme
            }
            b'*' => {
                if let Some(offset) = (&self.buf[self.pos..])
                    .windows(2)
                    .position(|word| word == b"*/")
                {
                    let lexeme = &self.buf[self.pos..self.pos + offset];
                    self.pos += offset + 2;
                    String::from_utf8(lexeme.to_vec()).expect("Error: Non utf8 input string")
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

    fn read_operator(&mut self) -> Result<Token, LexerError> {
        let pos = self.pos;
        let lexeme = self.read_while(is_operator);

        if consts::OPERATORS.contains(&lexeme.as_str()) {
            Ok(Token {
                tag: Tag::Operator,
                lexeme,
            })
        } else {
            Err(LexerError::MalformedToken(pos))
        }
    }

    fn read_token(&mut self) -> Result<Token, LexerError> {
        let pos = self.pos;

        match self.peek_u8(0) {
            Some(b'/') if self.peek_u8(1) == Some(b'/') || self.peek_u8(1) == Some(b'*') => {
                self.read_comment()
            }
            Some(b) if b.is_ascii_digit() => self.read_num(),
            Some(b) if b.is_ascii_alphabetic() => self.read_word(),
            Some(b) if is_operator(b) => self.read_operator(),
            Some(_) => Err(LexerError::UnexpectedByte(pos)),
            None => Err(LexerError::UnexpectedEof),
        }
    }

    fn read_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        self.skip_whitespaces();

        while !self.eof() {
            tokens.push(self.read_token()?);
            self.skip_whitespaces();
        }

        println!("{:?}", tokens);

        Ok(tokens)
    }
}

fn lex(input: &[u8]) -> Vec<Token> {
    let mut lexer = Lexer::new(input);

    lexer.read_tokens().unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        process::exit(1);
    })
}

fn is_operator(byte: u8) -> bool {
    match byte {
        b'+' | b'-' | b'*' | b'/' | b'<' | b'>' | b'=' | b'!' | b';' => true,
        _ => false,
    }
}

#[test]
fn lex_test() {
    assert!(lex(b"31 + 28 + 59").len() == 5);
    assert!(lex(b"count = count + increment;").len() == 6);
    assert!(lex(b"1.0 >= 2.0").len() == 3);
    assert!(lex(b"/* this is a comment */ true == false // another comment").len() == 5);
}
