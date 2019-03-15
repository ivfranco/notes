use std::fmt::{self, Formatter};

pub fn pad(indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:width$}", "", width = indent)
}
