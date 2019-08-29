pub mod ngram;
pub mod perplexity;

pub const GREAT_EXPECTIONS: &str = "./text/Great_Expections.txt";
pub const MOBY_DICK: &str = "./text/Moby_Dick.txt";

use std::{
    io::{self, Read},
    mem,
};

const PUNCUATIONS: &[u8] = b".,?!";

pub struct Tokenizer<R> {
    token: String,
    reader: R,
}

impl<R> Tokenizer<R>
where
    R: Read,
{
    fn new(reader: R) -> Self {
        Tokenizer {
            token: String::new(),
            reader,
        }
    }

    fn emit(&mut self) -> Option<String> {
        if self.token.is_empty() {
            None
        } else {
            let word = mem::replace(&mut self.token, String::new());
            Some(word)
        }
    }

    fn read_byte(&mut self) -> io::Result<Option<String>> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        let [byte] = buf;
        match byte {
            // letters
            _ if byte.is_ascii_alphabetic() || byte == b'\'' => {
                self.token.push(char::from(byte));
                Ok(None)
            }
            // meaningful puncuation
            _ if PUNCUATIONS.contains(&byte) => {
                let word = self.emit();
                self.token.push(char::from(byte));
                Ok(word)
            }
            // meaningless puncuation and whitespace
            _ => Ok(self.emit()),
        }
    }

    fn read_word(&mut self) -> Option<String> {
        loop {
            match self.read_byte() {
                Err(err) => {
                    if err.kind() != io::ErrorKind::UnexpectedEof {
                        eprintln!("{:?}", err);
                    }
                    return None;
                }
                Ok(None) => (),
                Ok(Some(word)) => return Some(word),
            }
        }
    }
}

impl<R> Iterator for Tokenizer<R>
where
    R: Read,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_word()
    }
}

pub fn tokenize<R>(reader: R) -> Tokenizer<R>
where
    R: Read,
{
    Tokenizer::new(reader)
}

pub fn words<R>(reader: R) -> impl Iterator<Item = String>
where
    R: Read,
{
    tokenize(reader).filter(|word| {
        PUNCUATIONS
            .iter()
            .all(|&p| !word.starts_with(char::from(p)))
    })
}

#[test]
fn tokenize_test() -> io::Result<()> {
    use std::fs::File;
    use std::io::BufReader;

    let text_file = File::open(GREAT_EXPECTIONS)?;
    let reader = BufReader::new(text_file);
    for _word in tokenize(reader).take(100) {
        // println!("{}", _word);
    }

    Ok(())
}
