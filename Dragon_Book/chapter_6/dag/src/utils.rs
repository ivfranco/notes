use std::fmt::{self, Formatter};

pub fn pad(indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:width$}", "", width = indent)
}

pub struct Array {
    base: usize,
    width: usize,
    dims: Vec<(usize, usize)>,
}

impl Array {
    pub fn new(base: usize, width: usize, dims: &[(usize, usize)]) -> Self {
        Array {
            base,
            width,
            dims: dims.to_owned(),
        }
    }

    pub fn assert_indices(&self, indices: &[usize]) {
        assert_eq!(self.dims.len(), indices.len());
        assert!(self
            .dims
            .iter()
            .zip(indices)
            .all(|((l, h), i)| i <= h && i >= l));
    }

    pub fn row_major(&self, indices: &[usize]) -> usize {
        self.assert_indices(indices);

        let offset = indices
            .iter()
            .zip(self.dims.iter())
            .fold(0, |idx, (i, (l, h))| idx * (h - l + 1) + (i - l));

        self.base + (offset * self.width)
    }

    pub fn col_major(&self, indices: &[usize]) -> usize {
        self.assert_indices(indices);

        let offset = indices
            .iter()
            .rev()
            .zip(self.dims.iter().rev())
            .fold(0, |idx, (i, (l, h))| idx * (h - l + 1) + (i - l));

        self.base + (offset * self.width)
    }
}

#[test]
fn location_test() {
    const INT_WIDTH: usize = 4;
    let arr = Array::new(0, INT_WIDTH, &[(1, 2), (1, 3)]);

    assert_eq!(arr.row_major(&[2, 2]), 4 * arr.width);
    assert_eq!(arr.col_major(&[2, 2]), 3 * arr.width);
}
