//! TODO: would be much cleaner if attribute list is a type that's both a set and sorted vector
#![allow(clippy::many_single_char_names, non_snake_case)]
mod sorted_uvec;

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    error::ParseError,
    multi::separated_list1,
    AsChar, IResult, InputTakeAtPosition, Parser,
};
use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FD {
    pub source: HashSet<u32>,
    pub target: HashSet<u32>,
}

impl FD {
    pub fn new<I, J, A, B>(source: I, target: J) -> Self
    where
        I: IntoIterator<Item = A>,
        J: IntoIterator<Item = B>,
        A: Borrow<u32>,
        B: Borrow<u32>,
    {
        let source: HashSet<_> = source.into_iter().map(|v| *v.borrow()).collect();
        let target: HashSet<_> = target
            .into_iter()
            // construct completely non-trivial FDs only
            .map(|v| *v.borrow())
            .filter(|v| !source.contains(v))
            .collect();
        Self { source, target }
    }

    pub fn is_deformed(&self) -> bool {
        self.source.is_empty() || self.target.is_empty()
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

        sep_by_comma(&to_sorted_vec(&self.fd.source), f)?;
        write!(f, " -> ")?;
        sep_by_comma(&to_sorted_vec(&self.fd.target), f)?;
        Ok(())
    }
}

pub fn closure_of<I, T>(attrs: I, dependencies: &[FD]) -> HashSet<u32>
where
    I: IntoIterator<Item = T>,
    T: Borrow<u32>,
{
    let mut closure: HashSet<_> = attrs.into_iter().map(|v| *v.borrow()).collect();
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

#[derive(Debug)]
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

#[derive(Default)]
pub struct NameRegister {
    cnt: u32,
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

    pub fn attrs(&self) -> HashSet<u32> {
        (0..self.cnt).collect()
    }

    pub fn sorted_attrs(&self) -> Vec<u32> {
        (0..self.cnt).collect()
    }

    pub fn categorize<I, T>(&self, attrs: I, dependencies: &[FD]) -> Category
    where
        I: IntoIterator<Item = T> + Copy,
        T: Borrow<u32>,
    {
        let closure = closure_of(attrs, dependencies);
        if !closure.is_superset(&self.attrs()) {
            return Category::Nonkey;
        }

        if attrs
            .into_iter()
            .map(|v| attrs.into_iter().filter(move |u| u.borrow() != v.borrow()))
            .any(|i| closure_of(i, dependencies).is_superset(&self.attrs()))
        {
            Category::Superkey
        } else {
            Category::Key
        }
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

    pub fn with_names<'a>(&'a self, attrs: &'a [u32]) -> AttrWithNames<'a> {
        AttrWithNames {
            attrs,
            register: self,
        }
    }

    pub fn parse(&self, input: &str) -> Option<FD> {
        let (_, (source, target)) = fd(input).ok()?;
        let source: Vec<u32> = source
            .iter()
            .map(|v| self.resolve(v))
            .collect::<Option<_>>()?;
        let target: Vec<u32> = target
            .iter()
            .map(|v| self.resolve(v))
            .collect::<Option<_>>()?;

        Some(FD::new(&source, &target))
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

fn ident(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn lexeme<P, I, O, E>(mut parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    P: Parser<I, O, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, _) = space0(input)?;
        parser.parse(input)
    }
}

// FD <- IDENT ("," IDENT)* "->" IDENT ("," IDENT)*
fn fd(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, source) = separated_list1(lexeme(tag(",")), lexeme(ident))(input)?;
    let (input, _) = lexeme(tag("->"))(input)?;
    let (input, target) = separated_list1(lexeme(tag(",")), lexeme(ident))(input)?;

    Ok((input, (source, target)))
}

fn to_sorted_vec<T: Ord + Copy>(set: &HashSet<T>) -> Vec<T> {
    let mut vec: Vec<_> = set.iter().copied().collect();
    vec.sort_unstable();
    vec
}

pub fn implies(FDs: &[FD], fd: &FD) -> bool {
    closure_of(&fd.source, FDs).is_superset(&fd.target)
}

fn all_subsets_of(attrs: &[u32]) -> impl Iterator<Item = Vec<u32>> + '_ {
    (0..=attrs.len()).flat_map(move |k| attrs.iter().copied().combinations(k))
}

fn project_to(attrs: &HashSet<u32>, FDs: &[FD]) -> Vec<FD> {
    let FDs: Vec<FD> = all_subsets_of(&to_sorted_vec(attrs))
        .map(|selected| {
            let mut closure = closure_of(&selected, FDs);
            closure.retain(|v| attrs.contains(v));
            FD::new(selected, closure)
        })
        .filter(|fd| !fd.is_deformed())
        .collect();

    minify(&FDs)
}

fn minify(FDs: &[FD]) -> Vec<FD> {
    FDs.iter()
        .filter(|fd| {
            !FDs.iter().any(|other| {
                other.source.len() < fd.source.len()
                    && other.source.is_subset(&fd.source)
                    && other.target.is_superset(&fd.target)
            })
        })
        .cloned()
        .collect()
}

fn violate<'a>(rel: &HashSet<u32>, FDs: &'a [FD]) -> Option<&'a FD> {
    FDs.iter()
        .find(|fd| !closure_of(&fd.source, FDs).is_superset(rel))
}

pub fn bcnf_decomposition<I, T>(rel: I, FDs: &[FD]) -> Vec<HashSet<u32>>
where
    I: IntoIterator<Item = T>,
    T: Borrow<u32>,
{
    let rel: HashSet<_> = rel.into_iter().map(|v| *v.borrow()).collect();
    let mut candidates: Vec<(HashSet<u32>, Cow<[FD]>)> = vec![(rel, Cow::Borrowed(FDs))];
    let mut bcnf: Vec<HashSet<u32>> = vec![];

    while let Some((rel, FDs)) = candidates.pop() {
        if let Some(fd) = violate(&rel, &FDs) {
            let rel_0 = closure_of(&fd.source, &FDs);
            let FDs_0 = project_to(&rel_0, &FDs);
            let rel_1 = &fd.source | &(&rel - &rel_0);
            let FDs_1 = project_to(&rel_1, &FDs);

            candidates.push((rel_0, Cow::Owned(FDs_0)));
            candidates.push((rel_1, Cow::Owned(FDs_1)));
        } else {
            bcnf.push(rel);
        }
    }

    bcnf
}

#[cfg(test)]
mod test {
    use nom::bitvec::prelude;

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
            to_sorted_vec(&closure_of(&[A, B], &dependencies)),
            &[A, B, C, D, E]
        );
        assert_eq!(to_sorted_vec(&closure_of(&[D], &dependencies)), &[D, E]);
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

        let FDs: Vec<_> = ["A -> B", "B -> C", "C -> D"]
            .iter()
            .map(|fd| reg.parse(fd).unwrap())
            .collect();

        let projection = project_to(&[A, C, D].iter().copied().collect(), &FDs);

        assert_eq!(projection.len(), 2);
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

        let FDs: Vec<_> = ["title, year -> studio_name", "studio_name -> president"]
            .iter()
            .map(|fd| reg.parse(fd).unwrap())
            .collect();

        assert_eq!(violate(&reg.attrs(), &FDs), Some(&FDs[1]));
    }

    #[test]
    fn bcnf_test() {
        let mut reg = NameRegister::new();
        let title = reg.register("title");
        let year = reg.register("year");
        let studio_name = reg.register("studio_name");
        let president = reg.register("president");
        let pres_addr = reg.register("pres_addr");

        let FDs: Vec<_> = [
            "title, year -> studio_name",
            "studio_name -> president",
            "president -> pres_addr",
        ]
        .iter()
        .map(|fd| reg.parse(fd).unwrap())
        .collect();

        let decomposition = bcnf_decomposition(reg.attrs(), &FDs);
        assert!(decomposition.contains(
            &[title, year, studio_name]
                .iter()
                .copied()
                .collect::<HashSet<_>>()
        ));

        assert!(decomposition.contains(
            &[studio_name, president]
                .iter()
                .copied()
                .collect::<HashSet<_>>()
        ));

        assert!(decomposition.contains(
            &[president, pres_addr]
                .iter()
                .copied()
                .collect::<HashSet<_>>()
        ));
    }
}
