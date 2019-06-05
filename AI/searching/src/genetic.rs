use rand::prelude::*;

pub trait Genetic {
    fn fitness(&self) -> u32;
    fn reproduce(&self, other: &Self) -> Self;
    fn mutate(&mut self);
    fn good_enough(&self) -> bool;
}

const SMALL_PROB: (u32, u32) = (1, 100);

pub fn genetic_search<N>(mut population: Vec<N>, cycles: usize) -> N
where
    N: Genetic + Clone,
{
    assert!(!population.is_empty());

    let mut rng = thread_rng();
    let (d, n) = SMALL_PROB;

    for _ in 0..cycles {
        let mut new_population = vec![];

        for _ in 0..population.len() {
            let x = population.choose_weighted(&mut rng, N::fitness).unwrap();
            let y = population.choose_weighted(&mut rng, N::fitness).unwrap();
            let mut child = x.reproduce(y);
            if rng.gen_ratio(d, n) {
                child.mutate();
            }

            if child.good_enough() {
                return child;
            }

            new_population.push(child);
        }

        population = new_population;
    }

    population.into_iter().max_by_key(N::fitness).unwrap()
}
