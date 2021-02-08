#![allow(clippy::many_single_char_names, non_snake_case)]

use std::{borrow::Borrow, collections::HashSet};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    error::ParseError,
    multi::separated_list1,
    AsChar, IResult, InputTakeAtPosition, Parser,
};

#[derive(PartialEq, Eq)]
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
    take_while1(|c: char| c.is_ascii_alphanumeric())(input)
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
}
