use num_traits::identities::Zero;

pub fn hill_climbing<N, C, FN, IN, FH>(start: &N, mut successors: FN, mut heuristic: FH) -> N
where
    N: Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FH: FnMut(&N) -> C,
{
    let mut curr = start.clone();

    while let Some(best) = successors(&curr)
        .into_iter()
        .min_by_key(|state| heuristic(state))
    {
        if heuristic(&best) < heuristic(&curr) {
            curr = best;
        } else {
            return curr;
        }
    }

    curr
}

#[test]
fn tsp_test() {
    use crate::tsp::{Map, TSP};
    use rand::prelude::*;

    let map: Map = random();
    let start = TSP::new(0, &map);

    let local_maxima = hill_climbing(&start, TSP::successors, TSP::heuristic);

    let optimal = start.solve();

    assert!(if local_maxima.is_goal() {
        optimal.cost() <= local_maxima.cost()
    } else {
        true
    });
}
