use indexmap::{IndexMap, IndexSet};
use random_fast_rng::{local_rng, Random};
use std::hash::Hash;

type Index = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Unk,
    Word(Index),
}

use Token::*;

impl Default for Token {
    fn default() -> Self {
        Token::Unk
    }
}

#[derive(Default)]
struct Dictionary {
    set: IndexSet<String>,
}

impl Dictionary {
    fn new() -> Self {
        Dictionary::default()
    }

    fn insert(&mut self, word: String) -> Index {
        let (index, _) = self.set.insert_full(word);
        index
    }

    fn get_token(&self, word: &str) -> Token {
        self.set
            .get_full(word)
            .map_or(Unk, |(index, _)| Word(index))
    }

    fn get(&self, index: Index) -> Option<&str> {
        self.set.get_index(index).map(|s| s.as_str())
    }
}

pub trait NGram: Copy + Eq + Hash {
    fn start() -> Self;
    fn len() -> usize;
    fn update(&mut self, token: Token);
}

pub type Unigram = ();

/// unigram is ndependent of prefixes
impl NGram for Unigram {
    fn start() -> Self {}

    fn len() -> usize {
        0
    }

    fn update(&mut self, _token: Token) {}
}

pub type Bigram = Token;

/// bigram depends on one preceeding word
impl NGram for Bigram {
    fn start() -> Self {
        Self::default()
    }

    fn len() -> usize {
        1
    }

    fn update(&mut self, token: Token) {
        *self = token;
    }
}

pub type Trigram = [Token; 2];

/// trigram depends on two preceeding words
impl NGram for Trigram {
    fn start() -> Self {
        Self::default()
    }

    fn len() -> usize {
        2
    }

    fn update(&mut self, token: Token) {
        *self = [self[1], token];
    }
}

pub struct Trainer<N> {
    dictionary: Dictionary,
    history: N,
    model: IndexMap<N, Vec<Index>>,
}

impl<N> Trainer<N>
where
    N: NGram,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn learn(&mut self, word: String) {
        let index = self.dictionary.insert(word);
        self.model
            .entry(self.history)
            .or_insert_with(Vec::new)
            .push(index);
        self.history.update(Word(index));
    }

    pub fn build(self) -> MarkovChain<N> {
        MarkovChain {
            dictionary: self.dictionary,
            model: self.model,
        }
    }
}

impl<N> Default for Trainer<N>
where
    N: NGram,
{
    fn default() -> Self {
        Trainer {
            dictionary: Dictionary::new(),
            history: N::start(),
            model: IndexMap::new(),
        }
    }
}

pub struct MarkovChain<N> {
    dictionary: Dictionary,
    model: IndexMap<N, Vec<Index>>,
}

impl<N> MarkovChain<N>
where
    N: NGram,
{
    fn random_state(&self) -> (N, &[Index]) {
        let mut rng = local_rng();
        let i = rng.get_usize() % self.model.len();
        let (history, succs) = self.model.get_index(i).unwrap();
        (*history, succs)
    }

    fn gen(&self, history: N) -> (String, N) {
        let mut rng = local_rng();

        let (mut history, succs) = if let Some(succs) = self.model.get(&history) {
            (history, succs.as_slice())
        } else {
            self.random_state()
        };

        let i = rng.get_usize() % succs.len();
        let index = succs[i];
        let word = self.dictionary.get(index).unwrap().to_string();
        history.update(Word(index));
        (word, history)
    }

    pub fn generator_with_seed<S>(&self, seed: &[S]) -> Generator<N>
    where
        S: AsRef<str>,
    {
        let history = seed.iter().fold(N::start(), |mut history, word| {
            history.update(self.dictionary.get_token(word.as_ref()));
            history
        });

        Generator {
            markov: self,
            history,
        }
    }

    pub fn generator(&self) -> Generator<N> {
        let (history, _) = self.random_state();
        Generator {
            markov: self,
            history,
        }
    }
}

pub struct Generator<'a, N> {
    markov: &'a MarkovChain<N>,
    history: N,
}

impl<'a, N> Iterator for Generator<'a, N>
where
    N: NGram,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let (word, next) = self.markov.gen(self.history);
        self.history = next;
        Some(word)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    #[test]
    fn generate_test() -> io::Result<()> {
        let mut trainer: Trainer<Trigram> = Trainer::new();
        let text_file = File::open(GREAT_EXPECTIONS)?;
        let reader = BufReader::new(text_file);

        for word in tokenize(reader) {
            trainer.learn(word);
        }

        let markov = trainer.build();

        // println!();
        for _word in markov.generator().take(200) {
            // print!("{} ", _word);
        }
        // println!();

        Ok(())
    }
}
