pub mod decision_list;
pub mod decision_tree;

type Value = u32;
type Attr = usize;
type Prob = f64;

pub struct Example {
    input: Vec<Value>,
    output: bool,
}

impl From<(Vec<Value>, bool)> for Example {
    fn from((input, output): (Vec<Value>, bool)) -> Self {
        Example { input, output }
    }
}

fn entropy(dist: &[Prob]) -> f64 {
    assert!(dist.iter().all(|p| !p.is_sign_negative()));
    assert!(dist.iter().sum::<f64>() <= 1.0);

    -dist
        .iter()
        .map(|p| {
            let log: f64 = p.log2();
            if log.is_finite() {
                p * log
            } else {
                // log2(p) overflowed, p must be 0.0
                0.0
            }
        })
        .sum::<f64>()
}

fn binary_entropy(p_true: Prob) -> f64 {
    entropy(&[p_true, 1.0 - p_true])
}

#[cfg(test)]
mod test {
    use super::*;

    const TOLERANCE: f64 = 0.0001;

    #[test]
    fn log_test() {
        // make sure that only 0.0 overflows log2
        assert!(std::f64::MIN_POSITIVE.log2().is_finite());
    }

    #[test]
    fn entropy_test() {
        assert!((entropy(&[0.5, 0.5]) - 1.0).abs() <= TOLERANCE);
        assert!((entropy(&[0.25; 4]) - 2.0).abs() <= TOLERANCE);
        assert!((entropy(&[0.0, 1.0])).abs() <= TOLERANCE);
    }
}