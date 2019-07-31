/// self reminder: be careful not to expose types from ndarray to the public interface
use ndarray::prelude::*;

pub type Prob = f64;
pub type Observation = usize;
pub type State = usize;

/// first order HMM with sensor Markov assumption.
#[derive(Debug)]
pub struct HMM {
    /// an n x n matrix T where:\
    /// Tij = P(Xk+1 = j | Xk = i)
    transition: Array2<Prob>,
    /// a sensor model in the form of a vector of matrices S where:\
    /// S[i]jj = P(Et = ei | Xt = j)\
    /// S[i]jk = 0 when j != k
    sensor: Vec<Array2<Prob>>,
}

impl HMM {
    /// expected inputs:\
    /// trans: transition model in row major\
    /// sensor_model: |E| x |X| sensor model in observation major
    pub fn new(trans: Vec<Prob>, sensor_model: Vec<Prob>) -> Self {
        let n = (trans.len() as Prob).sqrt().round() as usize;
        assert_eq!(
            n * n,
            trans.len(),
            "HMM::new: Transition model must be complete"
        );
        assert_eq!(
            sensor_model.len() % n,
            0,
            "HMM::new: sensor model must be complete"
        );

        // transpose the matrix as Array2::from_shape_vec expects inputs in column major
        let transition = Array2::from_shape_vec((n, n), trans)
            .unwrap()
            .reversed_axes();

        let sensor: Vec<_> = sensor_model
            .chunks_exact(n)
            .map(|chunk| {
                let mut array = Array2::zeros((n, n));
                for (i, d) in array.diag_mut().iter_mut().enumerate() {
                    *d = chunk[i];
                }
                array
            })
            .collect();

        HMM {
            transition,
            sensor,
        }
    }

    pub fn states(&self) -> usize {
        self.transition().dim().0
    }

    pub fn get_sensor(&self, o: Observation) -> ArrayView2<Prob> {
        self.sensor
            .get(o)
            .expect("HMM::get_sensor: observation value out of bound")
            .view()
    }

    pub fn transition(&self) -> ArrayView2<Prob> {
        self.transition.view()
    }
}

pub struct HMMContext<'a> {
    hmm: &'a HMM,
    forward: Vec<Array1<Prob>>,
    observations: Vec<Observation>,
}

impl<'a> HMMContext<'a> {
    pub fn new(hmm: &'a HMM, prior: Vec<f64>) -> Self {
        assert_eq!(prior.len(), hmm.states());

        let first_message = Array1::from_shape_vec(hmm.states(), prior).unwrap();

        HMMContext {
            hmm,
            forward: vec![first_message],
            // a dummy observation at t = 0
            observations: vec![0],
        }
    }

    /// Compute a new forward message given new observation.
    pub fn observe(&mut self, o: Observation) {
        let obv = self.hmm.get_sensor(o);
        let trs = self.hmm.transition().reversed_axes();
        let last = self.forward.last().unwrap();

        let mut fwd = obv.dot(&trs).dot(last);
        normalize(&mut fwd);
        self.forward.push(fwd);
        self.observations.push(o);
    }

    /// Return a forward message at time t.\
    /// May return None if the time t is out of bound.
    pub fn filter(&self, t: usize) -> Option<&[Prob]> {
        self.forward.get(t).and_then(|array| array.as_slice())
    }

    /// compute the seqeuence of P(Xk | e1:t) for 0 <= k <= t
    pub fn smooth(&self) -> Vec<Vec<Prob>> {
        // bt+1:t = 1
        let mut bak: Array1<Prob> = Array1::ones(self.hmm.states());
        let mut smoothing: Vec<Vec<Prob>> = Vec::with_capacity(self.forward.len());
        let trs = self.hmm.transition();

        for (k, fwd) in self.forward.iter().enumerate().rev() {
            let mut s = fwd * &bak;
            normalize(&mut s);
            smoothing.push(s.to_vec());

            // compute bk:t from bk+1:t
            bak = trs
                .dot(&self.hmm.get_sensor(self.observations[k]))
                .dot(&bak);
        }

        smoothing.reverse();
        smoothing
    }
}

#[allow(clippy::float_cmp)]
fn normalize(vector: &mut Array1<Prob>) {
    let sum = vector.sum();
    assert_ne!(
        sum, 0.0,
        "normalize: the sum of probabilities should not be zero"
    );
    *vector /= sum;
}

#[test]
fn rain_test() {
    const E: Prob = 0.001;
    // const BAREHAND: Observation = 0;
    const UMBRELLA: Observation = 1;

    // ~rain = 0, rain = 1
    let trans = vec![0.7, 0.3, 0.3, 0.7];

    // ~umbrella = 0, umbrella = 1
    let sensor_model = vec![0.8, 0.1, 0.2, 0.9];

    let prior = vec![0.5, 0.5];

    let hmm = HMM::new(trans, sensor_model);
    assert_eq!(hmm.transition.shape(), &[2, 2]);
    assert_eq!(hmm.sensor.len(), 2);
    assert!(hmm.sensor.iter().all(|s| s.shape() == [2, 2]));

    let mut context = HMMContext::new(&hmm, prior);
    context.observe(UMBRELLA);
    context.observe(UMBRELLA);

    let f2 = context.filter(2).unwrap();
    assert!((f2[0] - 0.117).abs() <= E);
    assert!((f2[1] - 0.883).abs() <= E);
    let smoothing = context.smooth();
    let s1 = &smoothing[1];
    assert!((s1[0] - 0.117).abs() <= E);
    assert!((s1[1] - 0.883).abs() <= E);
}
