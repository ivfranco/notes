#![allow(clippy::many_single_char_names, non_snake_case)]

pub mod chase;
mod parser;
mod sorted_uvec;

use itertools::Itertools;
use sorted_uvec::SortedUVec;
use std::{borrow::Borrow, mem, ops::Not};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

pub type Attrs = SortedUVec<u32>;

impl Attrs {
    pub fn with_names<'a>(&'a self, register: &'a NameRegister) -> AttrWithNames<'a> {
        AttrWithNames {
            attrs: self,
            register,
        }
    }

    pub fn keys(&self, FDs: &[FD]) -> Vec<Attrs> {
        all_subsets_of(self)
            .filter(|sub| categorize(sub, self, FDs) == Category::Key)
            .collect()
    }
}

pub fn attrs<I, J>(iter: I) -> Attrs
where
    I: IntoIterator<Item = J>,
    J: Borrow<u32>,
{
    Attrs::new(iter.into_iter().map(|v| *v.borrow()))
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct FD {
    pub source: Attrs,
    pub target: Attrs,
}

impl FD {
    pub fn new(source: Attrs, target: Attrs) -> Self {
        let target = &target - &source;
        Self { source, target }
    }

    pub fn is_deformed(&self) -> bool {
        self.source.is_empty() || self.target.is_empty()
    }

    pub fn split(&self) -> impl Iterator<Item = FD> + '_ {
        self.target
            .iter()
            .map(move |&v| FD::new(self.source.clone(), attrs(&[v])))
    }

    pub fn with_names<'a>(&'a self, register: &'a NameRegister) -> FDWithNames<'a> {
        FDWithNames { fd: self, register }
    }
}

pub struct FDWithNames<'a> {
    fd: &'a FD,
    register: &'a NameRegister,
}

impl Display for FDWithNames<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let sep_by_comma = |list: &[u32], f: &mut Formatter| -> fmt::Result {
            let mut first = true;
            for &v in list {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{}", self.register.name(v).unwrap_or("{Unnamed}"))?;
            }
            Ok(())
        };

        sep_by_comma(&*self.fd.source, f)?;
        write!(f, " -> ")?;
        sep_by_comma(&*self.fd.target, f)?;
        Ok(())
    }
}

pub fn closure_of(attrs: &Attrs, dependencies: &[FD]) -> Attrs {
    let mut closure = attrs.clone();
    let mut size = closure.len();

    loop {
        for fd in dependencies {
            if fd.source.is_subset(&closure) {
                closure.extend(fd.target.iter().copied());
            }
        }

        if closure.len() > size {
            size = closure.len();
        } else {
            break;
        }
    }

    closure
}

#[derive(Debug, PartialEq)]
pub enum Category {
    Nonkey,
    Key,
    Superkey,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

pub fn categorize(sub: &Attrs, rel: &Attrs, FDs: &[FD]) -> Category {
    let closure = closure_of(sub, FDs);
    if !closure.is_superset(&rel) {
        return Category::Nonkey;
    }

    let has_subkey = sub
        .iter()
        .map(|v| {
            let mut shirnked = sub.clone();
            shirnked.remove(v);
            shirnked
        })
        .any(|attrs| closure_of(&attrs, FDs).is_superset(&rel));

    if has_subkey {
        Category::Superkey
    } else {
        Category::Key
    }
}

#[derive(Default)]
pub struct NameRegister {
    pub cnt: u32,
    name_idx: HashMap<String, u32>,
    idx_name: HashMap<u32, String>,
}

impl NameRegister {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&self, name: &str) -> Option<u32> {
        self.name_idx.get(name).copied()
    }

    pub fn name(&self, idx: u32) -> Option<&str> {
        self.idx_name.get(&idx).map(|s| s.as_str())
    }

    pub fn attrs(&self) -> Attrs {
        (0..self.cnt).collect()
    }

    pub fn categorize(&self, attrs: &Attrs, dependencies: &[FD]) -> Category {
        categorize(attrs, &self.attrs(), dependencies)
    }

    pub fn register(&mut self, name: &str) -> u32 {
        self.resolve(name).unwrap_or_else(|| {
            let key = self.cnt;
            self.cnt += 1;
            self.name_idx.insert(name.to_string(), key);
            self.idx_name.insert(key, name.to_string());
            key
        })
    }

    pub fn parse(&self, input: &str) -> Option<FD> {
        let (_, (source, target)) = parser::fd(input).ok()?;
        let source: Attrs = source
            .iter()
            .map(|v| self.resolve(v))
            .collect::<Option<_>>()?;
        let target: Attrs = target
            .iter()
            .map(|v| self.resolve(v))
            .collect::<Option<_>>()?;

        Some(FD::new(source, target))
    }
}

pub struct AttrWithNames<'a> {
    attrs: &'a [u32],
    register: &'a NameRegister,
}

impl<'a> Display for AttrWithNames<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut is_first = true;
        write!(f, "{{ ")?;
        for &attr in self.attrs {
            if !is_first {
                write!(f, ", ")?;
            }
            is_first = false;
            f.write_str(self.register.name(attr).unwrap_or("{Unnamed}"))?;
        }
        write!(f, " }}")?;

        Ok(())
    }
}

pub fn parse_dependencies(register: &NameRegister, FDs: &[&str]) -> Vec<FD> {
    FDs.iter().map(|fd| register.parse(fd).unwrap()).collect()
}

pub fn implies(FDs: &[FD], fd: &FD) -> bool {
    closure_of(&fd.source, FDs).is_superset(&fd.target)
}

pub fn all_subsets_of(attrs: &[u32]) -> impl Iterator<Item = Attrs> + '_ {
    (0..=attrs.len())
        .flat_map(move |k| attrs.iter().copied().combinations(k))
        .map(From::from)
}

pub fn project_to(attrs: &Attrs, FDs: &[FD]) -> Vec<FD> {
    let FDs: Vec<FD> = all_subsets_of(&*attrs)
        .map(|selected| {
            let closure = closure_of(&selected, FDs);
            FD::new(selected, &closure & attrs)
        })
        .filter(|fd| !fd.is_deformed())
        .collect();

    minify(&FDs)
}

pub fn is_minimal_basis(FDs: &[FD]) -> bool {
    let mut FDs: Vec<_> = FDs.iter().flat_map(|fd| fd.split()).collect();
    !remove_implied(&mut FDs)
}

pub fn minify(FDs: &[FD]) -> Vec<FD> {
    let mut FDs: Vec<_> = FDs.iter().flat_map(|fd| fd.split()).collect();

    loop {
        let refined = remove_implied(&mut FDs);
        let shrinked = remove_redundant(&mut FDs);
        if !(refined || shrinked) {
            break;
        }
    }

    FDs.sort_by(|a, b| a.source.cmp(&b.source));
    FDs
}

fn remove_implied(FDs: &mut Vec<FD>) -> bool {
    for i in 0..FDs.len() {
        let FD = mem::take(&mut FDs[i]);
        if implies(FDs, &FD) {
            FDs.swap_remove(i);
            return true;
        }
        FDs[i] = FD;
    }

    false
}

fn remove_redundant(FDs: &mut [FD]) -> bool {
    for i in 0..FDs.len() {
        let FD = &FDs[i];
        for v in &FD.source {
            let mut shrinked = FD.clone();
            shrinked.source.remove(v);
            if implies(FDs, &shrinked) {
                FDs[i] = shrinked;
                return true;
            }
        }
    }

    false
}

pub fn all_violations<'a>(rel: &'a Attrs, FDs: &'a [FD]) -> impl Iterator<Item = &'a FD> + 'a {
    FDs.iter()
        .filter(move |fd| closure_of(&fd.source, FDs).is_superset(rel).not())
}

fn violation<'a>(rel: &'a Attrs, FDs: &'a [FD]) -> Option<&'a FD> {
    all_violations(rel, FDs).next()
}

pub fn is_bcnf_violation(rel: &Attrs, FDs: &[FD]) -> bool {
    violation(rel, FDs).is_some()
}

pub fn bcnf_decomposition(rel: &Attrs, FDs: &[FD]) -> Vec<Attrs> {
    let rel: Attrs = rel.clone();
    let mut candidates: Vec<(Attrs, Vec<FD>)> = vec![(rel, FDs.to_vec())];
    let mut bcnf: Vec<Attrs> = vec![];

    while let Some((rel, FDs)) = candidates.pop() {
        // every 2-attribute relation is in BCNF
        if rel.len() <= 2 {
            bcnf.push(rel);
            continue;
        }

        if let Some(fd) = violation(&rel, &FDs) {
            let rel_0 = closure_of(&fd.source, &FDs);
            let FDs_0 = project_to(&rel_0, &FDs);
            let rel_1 = &fd.source | &(&rel - &rel_0);
            let FDs_1 = project_to(&rel_1, &FDs);

            candidates.push((rel_0, FDs_0));
            candidates.push((rel_1, FDs_1));
        } else {
            bcnf.push(rel);
        }
    }

    bcnf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn closure_test() {
        let mut reg = NameRegister::new();

        let A = reg.register("A");
        let B = reg.register("B");
        let C = reg.register("C");
        let D = reg.register("D");
        let E = reg.register("E");
        let _F = reg.register("F");

        let dependencies = ["A, B -> C", "B, C -> A, D", "D -> E", "C, F -> B"]
            .iter()
            .map(|fd| {
                reg.parse(fd)
                    .expect("unregistered attributes or FD syntax error")
            })
            .collect::<Vec<_>>();

        assert_eq!(
            &*closure_of(&attrs(&[A, B]), &dependencies),
            &[A, B, C, D, E]
        );
        assert_eq!(&*closure_of(&attrs(&[D]), &dependencies), &[D, E]);
    }

    #[test]
    fn format_test() {
        let mut reg = NameRegister::new();
        reg.register("A");
        reg.register("B");
        reg.register("C");
        reg.register("D");

        let fd = reg.parse("B, A -> D, C").unwrap();
        assert_eq!(format!("{}", fd.with_names(&reg)), "A, B -> C, D");
    }

    #[test]
    fn project_test() {
        let mut reg = NameRegister::new();
        let A = reg.register("A");
        let _B = reg.register("B");
        let C = reg.register("C");
        let D = reg.register("D");

        let FDs = parse_dependencies(&reg, &["A -> B", "B -> C", "C -> D"]);

        let projection = project_to(&[A, C, D].iter().copied().collect(), &FDs);

        assert_eq!(projection.len(), 2);
        assert!(projection.iter().all(|fd| fd.target.len() == 1));
        assert!(implies(&projection, &reg.parse("A -> C, D").unwrap()));
        assert!(implies(&projection, &reg.parse("C -> D").unwrap()));
    }

    #[test]
    fn violation_test() {
        let mut reg = NameRegister::new();
        let _title = reg.register("title");
        let _year = reg.register("year");
        let _studio_name = reg.register("studio_name");
        let _president = reg.register("president");

        let FDs = parse_dependencies(
            &reg,
            &["title, year -> studio_name", "studio_name -> president"],
        );

        assert_eq!(violation(&reg.attrs(), &FDs), Some(&FDs[1]));
    }

    #[test]
    fn bcnf_test() {
        let mut reg = NameRegister::new();
        let title = reg.register("title");
        let year = reg.register("year");
        let studio_name = reg.register("studio_name");
        let president = reg.register("president");
        let pres_addr = reg.register("pres_addr");

        let FDs = parse_dependencies(
            &reg,
            &[
                "title, year -> studio_name",
                "studio_name -> president",
                "president -> pres_addr",
            ],
        );

        let decomposition = bcnf_decomposition(&reg.attrs(), &FDs);
        assert_eq!(decomposition.len(), 3);
        assert!(decomposition.contains(&attrs(&[title, year, studio_name])));
        assert!(decomposition.contains(&attrs(&[studio_name, president])));
        assert!(decomposition.contains(&attrs(&[president, pres_addr])));
    }
}
