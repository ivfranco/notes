use std::{
    cmp::{self, Ordering},
    collections::{BinaryHeap, HashMap, VecDeque},
};

pub fn delay_and_deviation(send: &[u32], recv: &[u32], u: f64) -> (Vec<f64>, Vec<f64>) {
    let init_delay = f64::from(recv[0] - send[0]);
    let init_deviation = 0.0;

    send.iter()
        .zip(recv)
        .skip(1)
        .scan((init_delay, init_deviation), |(d, v), (t, r)| {
            *d = (1.0 - u) * *d + u * f64::from(r - t);
            *v = (1.0 - u) * *v + u * (f64::from(r - t) - *d).abs();
            Some((*d, *v))
        })
        .unzip()
}

pub trait Queue {
    fn push(&mut self, id: usize, class: u32);
    fn pop(&mut self) -> Option<usize>;
}

impl Queue for VecDeque<usize> {
    fn push(&mut self, id: usize, _class: u32) {
        self.push_back(id)
    }

    fn pop(&mut self) -> Option<usize> {
        self.pop_front()
    }
}

#[derive(PartialEq, Eq)]
pub struct Pair {
    id: usize,
    class: u32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Pair {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.class
            .cmp(&rhs.class)
            .reverse()
            .then(self.id.cmp(&rhs.id).reverse())
    }
}

impl Queue for BinaryHeap<Pair> {
    fn push(&mut self, id: usize, class: u32) {
        self.push(Pair { id, class });
    }

    fn pop(&mut self) -> Option<usize> {
        self.pop().map(|pair| pair.id)
    }
}

pub struct RoundRobin {
    curr: u32,
    size: u32,
    pile: HashMap<u32, VecDeque<usize>>,
}

impl RoundRobin {
    pub fn new(size: u32) -> Self {
        Self {
            curr: 0,
            size,
            pile: HashMap::new(),
        }
    }
}

impl Queue for RoundRobin {
    fn push(&mut self, id: usize, class: u32) {
        self.pile.entry(class).or_default().push_back(id);
    }

    fn pop(&mut self) -> Option<usize> {
        for _ in 0..self.size {
            if let Some(id) = self
                .pile
                .get_mut(&self.curr)
                .and_then(|deque| deque.pop_front())
            {
                return Some(id);
            }

            // if queue of the current class is empty, immediately try the next class
            self.curr = (self.curr + 1) % self.size;
        }

        None
    }
}

pub struct WFQ {
    fst_deque: VecDeque<usize>,
    snd_deque: VecDeque<usize>,
    fst_served: u32,
    snd_served: u32,
    ratio: f64,
}

impl WFQ {
    pub fn new(ratio: f64) -> Self {
        Self {
            fst_deque: VecDeque::new(),
            snd_deque: VecDeque::new(),
            fst_served: 0,
            snd_served: 0,
            ratio,
        }
    }

    fn next_class(&self) -> u32 {
        let fst = f64::from(self.fst_served + 1) / f64::from(self.snd_served);
        let snd = f64::from(self.fst_served) / f64::from(self.snd_served + 1);

        if (fst - self.ratio).abs() <= (snd - self.ratio).abs() {
            0
        } else {
            1
        }
    }
}

impl Queue for WFQ {
    fn push(&mut self, id: usize, class: u32) {
        match class {
            0 => self.fst_deque.push_back(id),
            1 => self.snd_deque.push_back(id),
            _ => unreachable!(),
        }
    }

    fn pop(&mut self) -> Option<usize> {
        let (fst, snd) = if self.next_class() == 0 {
            (&mut self.fst_deque, &mut self.snd_deque)
        } else {
            (&mut self.snd_deque, &mut self.fst_deque)
        };

        if let Some(id) = fst.pop_front() {
            self.fst_served += 1;
            Some(id)
        } else if let Some(id) = snd.pop_front() {
            self.snd_served += 1;
            Some(id)
        } else {
            None
        }
    }
}

pub fn queue_simulate<Q>(mut queue: Q, arrivals: &[u32], classes: &[u32]) -> Vec<u32>
where
    Q: Queue,
{
    let mut departures = vec![0; arrivals.len()];
    let mut now = 0;

    for (id, (&t, &c)) in arrivals.iter().zip(classes).enumerate() {
        while now < t {
            now += 1;
            if let Some(id) = queue.pop() {
                departures[id] = now;
            }
        }

        queue.push(id, c);
    }

    while let Some(id) = queue.pop() {
        now += 1;
        departures[id] = now;
    }

    departures
}

pub struct LeakyBucket {
    queue: VecDeque<usize>,
    token: u32,
    rate: u32,
    size: u32,
}

impl LeakyBucket {
    pub fn new(rate: u32, size: u32) -> Self {
        Self {
            queue: VecDeque::new(),
            token: size,
            rate,
            size,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn push(&mut self, id: usize) {
        self.queue.push_back(id);
    }

    pub fn advance(&mut self) -> Vec<usize> {
        let mut departures = vec![];

        while self.token > 0 {
            if let Some(id) = self.queue.pop_front() {
                departures.push(id);
                self.token -= 1;
            } else {
                break;
            }
        }

        self.token = cmp::min(self.token + self.rate, self.size);
        departures
    }
}

impl std::fmt::Debug for LeakyBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "In queue: {:?}, tokens: {}", self.queue, self.token)?;
        Ok(())
    }
}

#[test]
fn queue_test() {
    assert_eq!(
        queue_simulate(
            VecDeque::new(),
            &[0, 0, 1, 1, 2, 3, 3, 5, 5, 7, 8, 8],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    )
}