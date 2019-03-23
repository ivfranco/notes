use std::fmt::{self, Debug, Formatter};

struct Chunk {
    offset: usize,
    size: usize,
}

impl Chunk {
    fn new(offset: usize, size: usize) -> Self {
        Chunk { offset, size }
    }

    fn fit(&self, object: usize) -> bool {
        self.size >= object
    }

    // return true if this Chunk is entirely consumed by the allocation
    fn alloc(&mut self, object: usize) -> bool {
        assert!(object <= self.size);

        self.size -= object;
        self.size < 8
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "range: [{}, {}], size: {}",
            self.offset,
            self.offset + self.size - 1,
            self.size
        )
    }
}

pub struct Heap {
    chunks: Vec<Chunk>,
}

impl Heap {
    pub fn new(sizes: &[usize]) -> Self {
        let chunks = sizes
            .iter()
            .scan(0, |offset, size| {
                let chunk = Chunk::new(*offset, *size);
                *offset += size;
                Some(chunk)
            })
            .collect();

        Heap { chunks }
    }

    pub fn first_fit(&mut self, object: usize) {
        let (i, chunk) = self
            .chunks
            .iter_mut()
            .enumerate()
            .find(|(_, chunk)| chunk.fit(object))
            .unwrap();

        if chunk.alloc(object) {
            self.chunks.remove(i);
        }
    }

    pub fn best_fit(&mut self, object: usize) {
        let (i, chunk) = self
            .chunks
            .iter_mut()
            .enumerate()
            .filter(|(_, chunk)| chunk.fit(object))
            .min_by_key(|(_, chunk)| chunk.size - object)
            .unwrap();

        if chunk.alloc(object) {
            self.chunks.remove(i);
        }
    }
}

impl Debug for Heap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        for chunk in &self.chunks {
            writeln!(f, "{:?}", chunk)?;
        }
        Ok(())
    }
}
