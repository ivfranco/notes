#[derive(Debug)]
pub enum BankerFailure {
    NotAvailable,
    MayDeadlock,
    MaxExceeded,
}

pub type BankerResult<T> = Result<T, BankerFailure>;

pub struct Banker {
    available: Vec<u32>,
    allocation: Vec<Vec<u32>>,
    pub need: Vec<Vec<u32>>,
}

impl Banker {
    pub fn new(available: Vec<u32>, need: Vec<Vec<u32>>) -> Self {
        let resources = available.len();
        let processes = need.len();

        Self {
            available,
            allocation: vec![vec![0; resources]; processes],
            need,
        }
    }

    pub fn from_state(available: Vec<u32>, allocation: Vec<Vec<u32>>, max: Vec<Vec<u32>>) -> Self {
        let need = allocation
            .iter()
            .zip(max.iter())
            .map(|(alloc, m)| alloc.iter().zip(m).map(|(r, n)| n - r).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            available,
            allocation,
            need,
        }
    }

    fn processes(&self) -> usize {
        self.need.len()
    }

    pub fn safe(&self) -> bool {
        let mut work = self.available.clone();
        let mut running = (0..self.processes()).collect::<Vec<_>>();

        while !running.is_empty() {
            let old_len = running.len();
            running.retain(|&p| {
                if self.need[p] <= work {
                    vector_add(&mut work, &self.allocation[p]);
                    false
                } else {
                    true
                }
            });
            if running.len() == old_len {
                return false;
            }
        }

        true
    }

    pub fn request(&mut self, process: usize, resources: &[u32]) -> BankerResult<()> {
        if resources > &self.need[process] {
            return Err(BankerFailure::MaxExceeded);
        }

        if resources > &self.available {
            return Err(BankerFailure::NotAvailable);
        }

        vector_sub(&mut self.available, resources);
        vector_add(&mut self.allocation[process], resources);
        vector_sub(&mut self.need[process], resources);

        if !self.safe() {
            vector_add(&mut self.available, resources);
            vector_sub(&mut self.allocation[process], resources);
            vector_add(&mut self.need[process], resources);
            Err(BankerFailure::MayDeadlock)
        } else {
            Ok(())
        }
    }
}

fn vector_add(lhs: &mut [u32], rhs: &[u32]) {
    for i in 0..lhs.len() {
        lhs[i] += rhs[i];
    }
}

fn vector_sub(lhs: &mut [u32], rhs: &[u32]) {
    for i in 0..lhs.len() {
        lhs[i] -= rhs[i];
    }
}
