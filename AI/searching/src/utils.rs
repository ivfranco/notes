use num_traits::Num;

pub fn possible_dests<N>(pos: N, width: N, size: N) -> Vec<N>
where
    N: Num + Ord + Copy,
{
    let mut dests = vec![];

    if pos >= width {
        dests.push(pos - width);
    }

    if size - pos > width {
        dests.push(pos + width);
    }

    if pos % width != N::zero() {
        dests.push(pos - N::one());
    }

    if pos % width != width - N::one() {
        dests.push(pos + N::one());
    }

    dests
}

pub fn diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
