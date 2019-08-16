pub mod decision_list;
pub mod decision_tree;

use std::borrow::Borrow;

type Value = u32;
type Attr = usize;
type Prob = f64;

pub struct Example {
    input: Vec<Value>,
    output: bool,
}

impl<V, C> From<(V, C)> for Example
where
    V: Into<Vec<Value>>,
    C: Into<bool>,
{
    fn from((input, output): (V, C)) -> Self {
        Example {
            input: input.into(),
            output: output.into(),
        }
    }
}

fn entropy(dist: &[Prob]) -> f64 {
    assert!(dist.iter().all(|&p| p >= 0.0));
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

fn class_count<I, B>(examples: I) -> (usize, usize)
where
    I: IntoIterator<Item = B>,
    B: Borrow<Example>,
{
    examples.into_iter().fold((0, 0), |(t, f), example| {
        if example.borrow().output {
            (t + 1, f)
        } else {
            (t, f + 1)
        }
    })
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use regex::Regex;

    pub const DATA: &str = "\
x1 Yes No No Yes Some $$$ No Yes French 0–10 y1 = Yes
x2 Yes No No Yes Full $ No No Thai 30–60 y2 = No
x3 No Yes No No Some $ No No Burger 0–10 y3 = Yes
x4 Yes No Yes Yes Full $ Yes No Thai 10–30 y4 = Yes
x5 Yes No Yes No Full $$$ No Yes French >60 y5 = No
x6 No Yes No Yes Some $$ Yes Yes Italian 0–10 y6 = Yes
x7 No Yes No No None $ Yes No Burger 0–10 y7 = No
x8 No No No Yes Some $$ Yes Yes Thai 0–10 y8 = Yes
x9 No Yes Yes No Full $ Yes No Burger >60 y9 = No
x10 Yes Yes Yes Yes Full $$$ No Yes Italian 10–30 y10 = No
x11 No No No No None $ No No Thai 0–10 y11 = No
x12 Yes Yes Yes Yes Full $ No No Burger 30–60 y12 = Yes";

    fn yes_no(s: &str) -> Value {
        match s {
            "Yes" => 1,
            "No" => 0,
            _ => unreachable!(),
        }
    }

    pub fn parse_examples(data: &str) -> Vec<Example> {
        let regex = Regex::new(
            r"(?x)
            x\d+\s
            (?P<alt>Yes|No)\s
            (?P<bar>Yes|No)\s
            (?P<fri>Yes|No)\s
            (?P<hun>Yes|No)\s
            (?P<pat>None|Some|Full)\s
            (?P<price>\$|\$\$|\$\$\$)\s
            (?P<rain>Yes|No)\s
            (?P<res>Yes|No)\s
            (?P<type>French|Thai|Burger|Italian)\s
            (?P<est>0–10|10–30|30–60|>60)\s
            y\d+\s=\s
            (?P<willwait>Yes|No)
        ",
        )
        .unwrap();

        data.lines()
            .map(|line| {
                let cap = regex.captures(line).unwrap();
                let mut input = vec![];
                input.push(yes_no(&cap["alt"]));
                input.push(yes_no(&cap["bar"]));
                input.push(yes_no(&cap["fri"]));
                input.push(yes_no(&cap["hun"]));
                input.push(match &cap["pat"] {
                    "None" => 0,
                    "Some" => 1,
                    "Full" => 2,
                    _ => unreachable!(),
                });
                input.push(match &cap["price"] {
                    "$" => 0,
                    "$$" => 1,
                    "$$$" => 2,
                    _ => unreachable!(),
                });
                input.push(yes_no(&cap["rain"]));
                input.push(yes_no(&cap["res"]));
                input.push(match &cap["type"] {
                    "French" => 0,
                    "Thai" => 1,
                    "Burger" => 2,
                    "Italian" => 3,
                    _ => unreachable!(),
                });
                input.push(match &cap["est"] {
                    "0–10" => 0,
                    "10–30" => 1,
                    "30–60" => 2,
                    ">60" => 3,
                    _ => unreachable!(),
                });
                let output = yes_no(&cap["willwait"]) != 0;

                (input, output).into()
            })
            .collect()
    }

    pub const INPUT_SCHEME: [Value; 10] = [
        2, // Yes, No
        2, // Yes, No
        2, // Yes, No
        2, // Yes, No
        3, // None, Some, Full,
        3, // $, $$, $$$
        2, // Yes, No
        2, // Yes, No
        4, // French, Thai, Burger, Italian
        4, // 0-10, 10-30, 30-60, >60
    ];

    const TOLERANCE: f64 = 1e-10;

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
