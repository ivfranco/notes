use std::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::{
    attrs, implies, minify, mvd::MVD, parse_FDs, parse_MVDs, project_to, Attrs, NameRegister, FD,
};
use fmt::Debug;
use itertools::Itertools;

const ORIGIN: u32 = 0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    fn max_by_revision(self, other: Self) -> Self {
        if self.revision > other.revision {
            self
        } else {
            other
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    fn new_uniform(cnt: u32, revision: u32) -> Self {
        let inner = (0..cnt)
            .map(|attr| Component::new(attr, revision))
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

impl Debug for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Index<u32> for Tuple {
    type Output = Component;

    fn index(&self, index: u32) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl IndexMut<u32> for Tuple {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.inner[index as usize]
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

struct Equation(Component, Component);

impl Equation {
    fn new(a: Component, b: Component) -> Self {
        assert_eq!(a.attr, b.attr);
        Self(a.min_by_revision(b), a.max_by_revision(b))
    }
}

#[derive(Clone, PartialEq)]
pub struct Tableau {
    inner: Vec<Tuple>,
}

impl Tableau {
    pub fn from_decomposition(decomposition: &[Attrs], cnt: u32) -> Self {
        let inner = decomposition
            .iter()
            .zip(1u32..)
            .map(|(attrs, i)| Tuple::new_chase(attrs, i, cnt))
            .collect();

        Self { inner }
    }

    pub fn from_source(cnt: u32, source: &Attrs) -> Self {
        let mut t1 = Tuple::new_uniform(cnt, 1);
        let mut t2 = Tuple::new_uniform(cnt, 2);

        for &v in source {
            t1[v].revision = ORIGIN;
            t2[v].revision = ORIGIN;
        }

        let inner = vec![t1, t2];
        Self { inner }
    }

    pub fn from_mvd(cnt: u32, mvd: &MVD) -> Self {
        let mut this = Self::from_source(cnt, &mvd.source);
        let rest = &(&attrs(0..cnt) - &mvd.source) - &mvd.target;
        for &v in &mvd.target {
            this[0][v].revision = ORIGIN;
        }
        for &v in &rest {
            this[1][v].revision = ORIGIN;
        }

        this
    }

    fn cross_tuples(&self) -> impl Iterator<Item = (&Tuple, &Tuple)> + '_ {
        self.iter()
            .enumerate()
            .cartesian_product(self.iter().enumerate())
            .filter_map(|((i, t0), (j, t1))| if i == j { None } else { Some((t0, t1)) })
    }

    fn deduce(&self, fd: &FD) -> Option<Equation> {
        self.cross_tuples()
            .filter(|(t0, t1)| fd.source.iter().all(|&v| t0[v] == t1[v]))
            .find_map(|(t0, t1)| {
                fd.target
                    .iter()
                    .find(|&&v| t0[v] != t1[v])
                    .map(|&v| Equation::new(t0[v], t1[v]))
            })
    }

    fn apply(&mut self, equation: &Equation) {
        let Equation(lhs, rhs) = equation;
        self.iter_mut().map(|t| &mut t[lhs.attr]).for_each(|c| {
            if c == rhs {
                *c = *lhs;
            }
        })
    }

    fn multiply(&mut self, mvd: &MVD) -> bool {
        let swapped: Vec<_> = self
            .cross_tuples()
            .filter(|(t0, t1)| {
                mvd.source.iter().all(|&v| t0[v] == t1[v])
                    && mvd.target.iter().any(|&v| t0[v] != t1[v])
            })
            .map(|(t0, t1)| {
                let mut t = t0.clone();
                for &v in &mvd.target {
                    t[v] = t1[v];
                }
                t
            })
            .collect();

        let len = self.len();
        self.extend(&swapped);
        len < self.len()
    }

    pub fn fixpoint(&mut self, FDs: &[FD]) -> bool {
        let mut refined = false;
        while let Some(equation) = FDs.iter().find_map(|fd| self.deduce(fd)) {
            refined = true;
            self.apply(&equation);
        }

        refined
    }

    pub fn contains_target(&self, attrs: &Attrs) -> bool {
        self.iter()
            .any(|t| attrs.iter().all(|&v| t[v].revision == ORIGIN))
    }

    pub fn contains_origin(&self) -> bool {
        self.iter().any(|t| t.is_original())
    }

    pub fn equated(&self, attrs: &Attrs) -> bool {
        // a Tableau is never empty
        let t0 = &self.inner[0];
        self.iter().all(|t| attrs.iter().all(|&v| t[v] == t0[v]))
    }
}

pub fn mix_implies_fd(cnt: u32, FDs: &[FD], MVDs: &[MVD], fd: &FD) -> bool {
    let mut tableau = Tableau::from_source(cnt, &fd.source);
    loop {
        let mut refined = false;
        refined = tableau.fixpoint(FDs) || refined;
        if tableau.equated(&fd.target) {
            return true;
        }
        for mvd in MVDs {
            refined = tableau.multiply(mvd) || refined;
        }
        if tableau.equated(&fd.target) {
            return true;
        }

        if !refined {
            return false;
        }
    }
}

pub fn mix_implies_mvd(cnt: u32, FDs: &[FD], MVDs: &[MVD], mvd: &MVD) -> bool {
    let mut tableau = Tableau::from_mvd(cnt, &mvd);
    // println!("{}", tableau);
    loop {
        let mut refined = false;
        refined = tableau.fixpoint(FDs) || refined;
        // println!("{}", tableau);
        if tableau.contains_origin() {
            return true;
        }
        for mvd in MVDs {
            refined = tableau.multiply(mvd) || refined;
        }
        // println!("{}", tableau);
        if tableau.contains_origin() {
            return true;
        }

        if !refined {
            return false;
        }
    }
}

impl Display for Tableau {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for tuple in &self.inner {
            writeln!(f, "{}", tuple)?;
        }
        Ok(())
    }
}

impl Debug for Tableau {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<'a> Extend<&'a Tuple> for Tableau {
    fn extend<T: IntoIterator<Item = &'a Tuple>>(&mut self, iter: T) {
        self.inner.extend(iter.into_iter().cloned());
        self.inner.sort_unstable();
        self.inner.dedup();
    }
}

impl Deref for Tableau {
    type Target = [Tuple];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl DerefMut for Tableau {
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

pub struct Dependencies {
    cnt: u32,
    FDs: Vec<FD>,
    MVDs: Vec<MVD>,
}

impl Dependencies {
    pub fn parse(register: &NameRegister, FDs: &[&str], MVDs: &[&str]) -> Self {
        let cnt = register.cnt();
        let FDs = parse_FDs(&register, FDs);
        let MVDs = parse_MVDs(&register, MVDs);

        Self { cnt, FDs, MVDs }
    }

    pub fn imply_fd(&self, fd: &FD) -> bool {
        mix_implies_fd(self.cnt, &self.FDs, &self.MVDs, fd)
    }

    pub fn imply_mvd(&self, mvd: &MVD) -> bool {
        mix_implies_mvd(self.cnt, &self.FDs, &self.MVDs, mvd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fixpoint_test() {
        let mut reg = NameRegister::new();
        let A = reg.register("A");
        let B = reg.register("B");
        let C = reg.register("C");
        let D = reg.register("D");

        let FDs = parse_FDs(&reg, &["B -> A, D"]);

        let decomposition = [attrs(&[A, B]), attrs(&[B, C]), attrs(&[C, D])];
        let mut rel = Tableau::from_decomposition(&decomposition, 4);

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
        let FDs = parse_FDs(&reg, &["theater -> city", "title, city -> theater"]);

        assert_eq!(not_preserved(&decomposition, &FDs), Some(&FDs[1]));
    }

    #[test]
    fn mix_implies_fd_test() {
        let mut reg = NameRegister::new();
        let _A = reg.register("A");
        let _B = reg.register("B");
        let _C = reg.register("C");
        let _D = reg.register("D");

        let FDs = parse_FDs(&reg, &["D -> C"]);
        let MVDs = parse_MVDs(&reg, &["A ->> B, C"]);

        let fd = reg.parse_fd("A -> C").unwrap();

        assert!(mix_implies_fd(reg.cnt(), &FDs, &MVDs, &fd));
        assert!(!mix_implies_fd(
            reg.cnt(),
            &FDs,
            &MVDs,
            &reg.parse_fd("B -> C").unwrap()
        ))
    }

    #[test]
    fn implies_fd_test() {
        let mut reg = NameRegister::new();
        let _A = reg.register("A");
        let _B = reg.register("B");
        let _C = reg.register("C");
        let _D = reg.register("D");
        let _E = reg.register("E");
        let _F = reg.register("F");

        let FDs = parse_FDs(&reg, &["A, B -> C", "B, C -> A, D", "D -> E", "C, F -> B"]);
        let fd = reg.parse_fd("A, B -> D").unwrap();

        assert!(mix_implies_fd(reg.cnt(), &FDs, &[], &fd));
    }

    #[test]
    fn mix_implies_mvd_test() {
        let mut reg = NameRegister::new();
        let _A = reg.register("A");
        let _B = reg.register("B");
        let _C = reg.register("C");
        let _D = reg.register("D");

        let FDs = parse_FDs(&reg, &["A -> B"]);
        let MVDs = parse_MVDs(&reg, &["B ->> C"]);

        assert!(mix_implies_mvd(
            reg.cnt(),
            &FDs,
            &MVDs,
            &reg.parse_mvd("A ->> C").unwrap()
        ));
        assert!(mix_implies_mvd(
            reg.cnt(),
            &FDs,
            &MVDs,
            &reg.parse_mvd("A ->> B, D").unwrap()
        ));
        assert!(mix_implies_mvd(
            reg.cnt(),
            &FDs,
            &MVDs,
            &reg.parse_mvd("A ->> B").unwrap()
        ));
        assert!(!mix_implies_mvd(
            reg.cnt(),
            &FDs,
            &MVDs,
            &reg.parse_mvd("B ->> A").unwrap()
        ));
    }
}
