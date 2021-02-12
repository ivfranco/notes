use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use crate::{implies, minify, project_to, Attrs, FD};
use fmt::Debug;

const ORIGIN: u32 = 0;

#[derive(Clone, Copy, PartialEq)]
pub struct Component {
    attr: u32,
    revision: u32,
}

impl PartialEq<(u32, u32)> for Component {
    fn eq(&self, other: &(u32, u32)) -> bool {
        self.attr == other.0 && self.revision == other.1
    }
}

impl Component {
    fn new(attr: u32, revision: u32) -> Self {
        Self { attr, revision }
    }

    fn min_by_revision(self, other: Self) -> Self {
        if self.revision > other.revision {
            other
        } else {
            self
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let letter: char = ('a' as u32 + self.attr).try_into().unwrap();
        let component = if self.revision == ORIGIN {
            format!("{}", letter)
        } else {
            format!("{}{}", letter, self.revision)
        };

        f.pad(&component)
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Tuple {
    inner: Vec<Component>,
}

impl Tuple {
    fn new_chase(attrs: &Attrs, idx: u32, cnt: u32) -> Self {
        let inner = (0..cnt)
            .map(|attr| {
                if attrs.contains(&attr) {
                    Component::new(attr, ORIGIN)
                } else {
                    Component::new(attr, idx)
                }
            })
            .collect();

        Self { inner }
    }

    fn is_original(&self) -> bool {
        self.iter().all(|c| c.revision == ORIGIN)
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, c) in self.inner.iter().enumerate() {
            if i + 1 != self.inner.len() {
                write!(f, "{:<4}|", c)?;
            } else {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

impl Deref for Tuple {
    type Target = [Component];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

#[derive(Clone, PartialEq)]
pub struct Relation {
    inner: Vec<Tuple>,
}

impl Relation {
    pub fn from_decomposition(decomposition: &[Attrs], cnt: u32) -> Self {
        let inner = decomposition
            .iter()
            .zip(1u32..)
            .map(|(attrs, i)| Tuple::new_chase(attrs, i, cnt))
            .collect();

        Self { inner }
    }

    fn deduce(&mut self, fd: &FD) {
        let len = self.len();

        for i in 0..len {
            for j in (0..len).filter(|&j| j != i) {
                let agree = fd.source.iter().all(|&k| {
                    let k = k as usize;
                    self[i][k] == self[j][k]
                });
                if agree {
                    for &k in &fd.target {
                        let k = k as usize;
                        // when applying FD, always overwrite the component with larger revision
                        // eventually the component with smaller revision (e.g. the original) will dominate
                        let c = self[i][k].min_by_revision(self[j][k]);
                        self[i][k] = c;
                        self[j][k] = c;
                    }
                }
            }
        }
    }

    pub fn fixpoint(&mut self, FDs: &[FD]) {
        loop {
            let orig = self.clone();
            for fd in FDs {
                self.deduce(fd);
            }
            if self == &orig {
                break;
            }
        }
    }

    pub fn contains_origin(&self) -> bool {
        self.iter().any(|t| t.is_original())
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for tuple in &self.inner {
            writeln!(f, "{}", tuple)?;
        }
        Ok(())
    }
}

impl Deref for Relation {
    type Target = [Tuple];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for Relation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

// FD not preserved after decomposition is FD that cannot be implied from the union of projected FDs
pub fn not_preserved<'a>(decomposition: &[Attrs], FDs: &'a [FD]) -> Option<&'a FD> {
    let recomposed: Vec<_> = decomposition
        .iter()
        .flat_map(|attrs| project_to(&attrs, FDs))
        .collect();
    let recomposed = minify(&recomposed);

    FDs.iter().find(|fd| !implies(&recomposed, fd))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{attrs, parse_dependencies, NameRegister};

    #[test]
    fn fixpoint_test() {
        let mut reg = NameRegister::new();
        let A = reg.register("A");
        let B = reg.register("B");
        let C = reg.register("C");
        let D = reg.register("D");

        let FDs = parse_dependencies(&reg, &["B -> A, D"]);

        let decomposition = [attrs(&[A, B]), attrs(&[B, C]), attrs(&[C, D])];
        let mut rel = Relation::from_decomposition(&decomposition, 4);

        rel.fixpoint(&FDs);

        assert_eq!(&*rel[0], [(A, 0), (B, 0), (C, 1), (D, 1)]);
        assert_eq!(&*rel[1], [(A, 0), (B, 0), (C, 0), (D, 1)]);
        assert_eq!(&*rel[2], [(A, 3), (B, 3), (C, 0), (D, 0)]);
    }

    #[test]
    fn preservation_test() {
        let mut reg = NameRegister::new();
        let title = reg.register("title");
        let city = reg.register("city");
        let theater = reg.register("theater");

        let decomposition = [attrs(&[theater, city]), attrs(&[theater, title])];
        let FDs = parse_dependencies(&reg, &["theater -> city", "title, city -> theater"]);

        assert_eq!(not_preserved(&decomposition, &FDs), Some(&FDs[1]));
    }
}
