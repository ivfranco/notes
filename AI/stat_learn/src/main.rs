use rand::{distributions::Bernoulli, prelude::*};
use textplots::{Chart, Plot, Shape};

fn main() {
    exercise_20_1();
    exercise_20_7();
}

mod candy {
    type DIST = [f64; 5];

    pub const PRIOR: DIST = [0.1, 0.2, 0.4, 0.2, 0.1];
    pub const HYPOTHESES: DIST = [1.0, 0.75, 0.5, 0.25, 0.0];

    fn normalize(dist: &mut DIST) {
        let sum = dist.iter().sum::<f64>();
        assert!(sum > 0.0);
        dist.iter_mut().for_each(|p| *p /= sum);
    }

    pub fn posterior(prior: &DIST, sample: bool) -> DIST {
        let mut posterior = [0.0; 5];
        for (i, pre) in prior.iter().enumerate() {
            posterior[i] = pre * if sample {
                HYPOTHESES[i]
            } else {
                1.0 - HYPOTHESES[i]
            };
        }
        normalize(&mut posterior);
        posterior
    }

    pub fn predict_lime(dist: &DIST) -> f64 {
        dist.iter()
            .zip(HYPOTHESES.iter())
            .map(|(p, h)| p * (1.0 - h))
            .sum()
    }
}

fn exercise_20_1() {
    use candy::*;

    let rng = thread_rng();

    println!("\n20.1");

    for &h in HYPOTHESES.iter() {
        println!("Hypothesis: p(cherry) = {:.2}, p(lime) = {:.2}", h, 1.0 - h);

        let sample: Vec<_> = rng
            .sample_iter(Bernoulli::new(h).unwrap())
            .take(101)
            .collect();
        
        let mut dist = PRIOR;
        let mut lines = vec![vec![]; 5];
        let mut prediction = vec![];

        for (i, s) in sample.into_iter().enumerate() {
            for (j, &p) in dist.iter().enumerate() {
                lines[j].push((i as f32, p as f32));
            }
            prediction.push((i as f32, predict_lime(&dist) as f32));
            dist = posterior(&dist, s);
        }

        println!("P(hi | d1, .., dN)");
        Chart::new(120, 60, 0.0, 100.0)
            .lineplot(Shape::Lines(&lines[0]))
            .lineplot(Shape::Lines(&lines[1]))
            .lineplot(Shape::Lines(&lines[2]))
            .lineplot(Shape::Lines(&lines[3]))
            .lineplot(Shape::Lines(&lines[4]))
            .display();
        
        println!("P(DN+1 = lime | d1, .., dN)");
        Chart::new(120, 60, 0.0, 100.0)
            .lineplot(Shape::Lines(&prediction))
            .display();
    }
}

fn exercise_20_7() {
    println!("\n20.7");

    const EPSILON: f32 = 0.001;
    Chart::new(120, 60, 0.0, 1.0)
        .lineplot(Shape::Continuous(|x| x.powf(EPSILON - 1.0) * (1.0 - x).powf(EPSILON - 1.0)))
        .display();
    Chart::new(120, 60, 0.0, 1.0)
        .lineplot(Shape::Continuous(|x| x.powf(EPSILON) * (1.0 - x).powf(EPSILON - 1.0)))
        .display();
}