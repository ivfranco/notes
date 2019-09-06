use markov::{HMM, HMMContext, Observation, Prob, State, normalize};
use itertools::Itertools;

fn main() {
    exercise_25_1();
}

fn exercise_25_1() {
    println!("\n25.1");

    const X: State = 4;
    const Z: Observation = 1;

    #[rustfmt::skip]
    let trans = vec![
        1.0, 0.0, 0.0, 0.0, // x1
        0.0, 1.0, 0.0, 0.0, // x2
        0.0, 0.0, 1.0, 0.0, // x3
        0.0, 0.0, 0.0, 1.0, // x4
    ];

    #[rustfmt::skip]
    let sensor_model = vec![
        0.2, 0.6, 0.9, 0.9, // ~z
        0.8, 0.4, 0.1, 0.1, // z
    ];

    let hmm = HMM::new(trans, sensor_model.clone());
    let mut context = HMMContext::new(&hmm, vec![1.0 / X as f64; 4]);
    context.observe(Z);
    let posterior = context.filter(1).unwrap();

    println!("P(X | N = ∞) = {:?}", posterior);

    for n in 1 ..= 10 {
        let mut dist = [0.0; X];
        let meta = vec![0usize ..= 3usize; n];
        // iterate through all the possible samples
        for sample in meta.into_iter().multi_cartesian_product() {
            let mut weights = vec![0.0; n];
            for (i, x) in sample.iter().enumerate() {
                weights[i] = sensor_model[x + X];
            }
            normalize(&mut weights);
            for (i, w) in weights.iter().enumerate() {
                dist[sample[i]] += w;
            }
        }
        normalize(&mut dist);
        println!("P(X | N = {}) = {:?}", n, dist);
        println!("KL = {:.6}", kl_divergence(&dist, posterior));
    }
}

fn kl_divergence(p: &[Prob], q: &[Prob]) -> f64 {
    p.iter()
        .zip(q)
        .map(|(&px, &qx)| if px == 0.0 || qx == 0.0 {
            0.0
        } else {
            px * (px / qx).ln()
        })
        .sum()
}