use std::{
    fs::File,
    io::{self, BufReader},
};
use trigram::{
    *,
    ngram::{self, Bigram, NGram, Trigram, Unigram},
    perplexity::MultiModel,
};

fn main() -> io::Result<()> {
    exercise_22_1()?;
    exercise_22_3()?;

    Ok(())
}

fn gen_words<N>() -> io::Result<()>
where
    N: NGram,
{
    use ngram::Trainer;

    let text_file = File::open(GREAT_EXPECTIONS)?;
    let reader = BufReader::new(text_file);
    let mut trainer: ngram::Trainer<N> = Trainer::new();
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

fn average_perplexity<I, S>(model: &MultiModel, words: I) -> f64
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut cnt = 0;
    let mut perplexity = 0.0;
    for word in words {
        perplexity += model.perplexity(word.as_ref());
        cnt += 1;
    }

    perplexity / f64::from(cnt)
}

fn exercise_22_3() -> io::Result<()> {
    println!("\n22.3");

    let great_expectations = BufReader::new(File::open(GREAT_EXPECTIONS)?);
    let moby_dick = BufReader::new(File::open(MOBY_DICK)?);

    let dickens_model = MultiModel::from_reader(great_expectations)?;
    let melville_model = MultiModel::from_reader(moby_dick)?;

    const QUOTE: &str = "It was the best of times
it was the worst of times
it was the age of wisdom
it was the age of foolishness
it was the epoch of belief
it was the epoch of incredulity
it was the season of Light
it was the season of Darkness
it was the spring of hope
it was the winter of despair
we had everything before us
we had nothing before us
we were all going direct to Heaven
we were all going direct the other way";

    let dickens_perplexity = average_perplexity(&dickens_model, QUOTE.split(|c| c == ' ' || c == '\n'));
    let melville_perplexity = average_perplexity(&melville_model, QUOTE.split(|c| c == ' ' || c == '\n'));

    dbg!(dickens_perplexity, melville_perplexity);
    assert!(dickens_perplexity < melville_perplexity);

    Ok(())
}