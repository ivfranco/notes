use trigram::{
    *,
    ngram::*,
};
use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    exercise_22_1()?;

    Ok(())
}

fn gen_words<N>() -> io::Result<()>
where
    N: NGram,
{
    let text_file = File::open(EXAMPLE_TEXT)?;
    let reader = BufReader::new(text_file);
    let mut trainer: Trainer<N> = Trainer::new();
    for word in tokenize(reader) {
        trainer.learn(word);
    }
    let markov = trainer.build();
    for word in markov.generator().take(200) {
        print!("{} ", word);
    }
    Ok(())
}

fn exercise_22_1() -> io::Result<()> {
    println!("\n22.1");

    println!("Unigram");
    gen_words::<Unigram>()?;
    println!("\nBigram");
    gen_words::<Bigram>()?;
    println!("\nTrigram");
    gen_words::<Trigram>()?;
    Ok(())
}