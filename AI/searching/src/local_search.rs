use ordered_float::OrderedFloat;
use rand::prelude::*;

type Cost = f64;

pub trait Local: Sized {
    fn heuristic(&self) -> Cost;
    fn successors(&self) -> Vec<Self>;
    fn successful(&self) -> bool;
}

pub fn steepest_ascent<N>(start: &N) -> N
where
    N: Local + Clone,
{
    let mut curr = start.clone();

    while let Some(best) = curr
        .successors()
        .into_iter()
        .min_by_key(|state| OrderedFloat(state.heuristic()))
    {
        if best.heuristic() < curr.heuristic() {
            curr = best;
        } else {
            return curr;
        }
    }

    curr
}

pub fn first_choice<N>(start: &N) -> N
where
    N: Local + Clone,
{
    let mut curr = start.clone();
    let mut rng = thread_rng();

    loop {
        let mut succs: Vec<_> = curr.successors();
        succs.shuffle(&mut rng);
        if let Some(first) = succs
            .into_iter()
            .find(|succ| succ.heuristic() < curr.heuristic())
        {
            curr = first;
        } else {
            return curr;
        }
    }
}

pub fn random_restart<N, F>(mut gen: F) -> N
where
    N: Local + Clone,
    F: FnMut() -> N,
{
    loop {
        let curr = gen();
        let local_maxima = steepest_ascent(&curr);
        if local_maxima.successful() {
            return local_maxima;
        }
    }
}

pub fn simulated_annealing<N, F>(start: &N, mut schedule: F) -> N
where
    N: Local + Clone,
    F: FnMut(usize) -> Cost,
{
    let mut curr = start.clone();
    let mut rng = thread_rng();
    let mut t = 0;

    loop {
        let temp = schedule(t);
        if temp <= 0.0 {
            return curr;
        }

        let next = curr.successors().choose(&mut rng).unwrap().clone();
        if next.heuristic() < curr.heuristic() {
            curr = next;
        } else {
            let delta = (next.heuristic() - curr.heuristic()) * temp;
            if random::<f64>() <= (-delta).exp() {
                curr = next;
            }
        }

        t += 1;
    }
}

#[test]
fn eight_queens_test() {
    use crate::eight_queens::Queens;
    use rand::prelude::*;

    let start: Queens = random();

    let steepest_ascent_maxima = steepest_ascent(&start);
    let first_choice_maxima = first_choice(&start);
    let random_restart_maxima: Queens = random_restart(random);
    let simulated_annealing_maxima = simulated_annealing(&start, |t| (1000 - 10 * t) as f64);
    let optimal = start.solve();

    assert!(optimal.is_goal());
    assert!(random_restart_maxima.is_goal());

    println!("Steepest ascent: {}", steepest_ascent_maxima.heuristic());
    println!("First choice: {}", first_choice_maxima.heuristic());
    println!(
        "Simulated annealing: {}",
        simulated_annealing_maxima.heuristic()
    );
}
