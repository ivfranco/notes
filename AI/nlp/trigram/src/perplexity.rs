use super::*;
use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Read, Seek, SeekFrom},
};

type Prob = f64;

const SPACE: u8 = b' ';

pub trait NGram: Copy + Eq + Hash {
    fn start() -> Self;
    fn update(&mut self, byte: u8);
}

type Unigram = ();
type Bigram = u8;
type Trigram = [u8; 2];

impl NGram for Unigram {
    fn start() -> Self {}

    fn update(&mut self, _byte: u8) {}
}

impl NGram for Bigram {
    fn start() -> Self {
        SPACE
    }

    fn update(&mut self, byte: u8) {
        *self = byte;
    }
}

impl NGram for Trigram {
    fn start() -> Self {
        [SPACE; 2]
    }

    fn update(&mut self, byte: u8) {
        *self = [self[1], byte];
    }
}

pub struct Trainer<N> {
    history: N,
    model: HashMap<N, Vec<(u32, u8)>>,
}

impl<N> Trainer<N>
where
    N: NGram,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn learn(&mut self, byte: u8) {
        let succs = self.model.entry(self.history).or_insert_with(Vec::new);
        if let Some((cnt, _)) = succs.iter_mut().find(|(_, succ)| *succ == byte) {
            *cnt += 1;
        } else {
            succs.push((1, byte));
        }

        self.history.update(byte);
    }

    pub fn learn_word(&mut self, word: &str) {
        for byte in word.bytes() {
            self.learn(byte);
        }
        self.learn(SPACE);
    }

    fn from_reader<R>(reader: R) -> Model<N>
    where
        R: Read,
    {
        let mut trainer = Trainer::new();
        for word in words(reader) {
            trainer.learn_word(&word);
        }
        trainer
            .model
            .into_iter()
            .map(|(prev, succs)| {
                let sum = succs.iter().map(|(cnt, _)| *cnt).sum();

                (prev, (sum, succs))
            })
            .collect()
    }
}

impl<N> Default for Trainer<N>
where
    N: NGram,
{
    fn default() -> Self {
        Trainer {
            history: N::start(),
            model: HashMap::new(),
        }
    }
}

type Model<N> = HashMap<N, (u32, Vec<(u32, u8)>)>;

fn query<N>(model: &Model<N>, history: N, byte: u8) -> Prob
where
    N: NGram,
{
    model
        .get(&history)
        .and_then(|(sum, succs)| {
            succs
                .iter()
                .find(|(_, succ)| *succ == byte)
                .map(|(cnt, _)| f64::from(*cnt) / f64::from(*sum))
        })
        .unwrap_or(0.0)
}

const TRI_COFF: f64 = 0.7;
const BI_COFF: f64 = 0.2;
const UNI_COFF: f64 = 0.1;

pub struct MultiModel {
    uni: Model<Unigram>,
    bi: Model<Bigram>,
    tri: Model<Trigram>,
}

impl MultiModel {
    pub fn from_reader<R>(mut reader: R) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        reader.seek(SeekFrom::Start(0))?;
        let uni = Trainer::<Unigram>::from_reader(&mut reader);
        reader.seek(SeekFrom::Start(0))?;
        let bi = Trainer::<Bigram>::from_reader(&mut reader);
        reader.seek(SeekFrom::Start(0))?;
        let tri = Trainer::<Trigram>::from_reader(&mut reader);

        Ok(MultiModel { uni, bi, tri })
    }

    fn multi_query(&self, history: Trigram, byte: u8) -> Prob {
        TRI_COFF * query(&self.tri, history, byte)
            + BI_COFF * query(&self.bi, history[1], byte)
            + UNI_COFF * query(&self.uni, (), byte)
    }

    pub fn multi_query_word(&self, word: &str) -> Prob {
        let mut history = Trigram::start();
        let mut prob = 1.0;
        for byte in word.bytes() {
            prob *= self.multi_query(history, byte);
            history.update(byte);
        }
        prob *= self.multi_query(history, SPACE);
        prob
    }

    pub fn perplexity(&self, word: &str) -> f64 {
        let mut history = Trigram::start();
        let mut perplexity = 1.0;
        for byte in word.bytes() {
            perplexity *= self
                .multi_query(history, byte)
                .powf(-1.0 / word.len() as f64);
            history.update(byte);
        }
        perplexity *= self.multi_query(history, SPACE);
        perplexity
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn perplexity_test() -> io::Result<()> {
        let text_file = File::open(GREAT_EXPECTIONS)?;
        let reader = BufReader::new(text_file);
        let model = MultiModel::from_reader(reader)?;

        let fully_segmented = model.perplexity(
            "the longest list of the longest stuff at the longest domain name at long last",
        );
        let less_segmented = model.perplexity(
            "the longest list of the longest stuff atthe longest domain name at long last",
        );
        let over_segmented = model.perplexity(
            "the longest list of the long est stuff at the longest domain name at long last",
        );

        assert!(fully_segmented < less_segmented);
        assert!(fully_segmented < over_segmented);
        Ok(())
    }
}
