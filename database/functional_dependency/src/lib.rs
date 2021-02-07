#![allow(clippy::many_single_char_names, non_snake_case)]

use std::collections::HashSet;
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
    source: HashSet<u32>,
    target: HashSet<u32>,
}

impl FD {
    pub fn new(source: &[u32], target: &[u32]) -> Self {
        let source: HashSet<_> = source.iter().copied().collect();
        let target: HashSet<_> = target
            .iter()
            // construct completely non-trivial FDs only
            .filter(|v| !source.contains(v))
            .copied()
            .collect();
        Self { source, target }
    }

    pub fn with_names<'a>(&'a self, register: &'a NameRegister) -> WithNames<'a> {
        WithNames { fd: self, register }
    }
}

pub struct WithNames<'a> {
    fd: &'a FD,
    register: &'a NameRegister,
}

impl Display for WithNames<'_> {
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

pub fn closure_of(set: &[u32], dependencies: &[FD]) -> HashSet<u32> {
    let mut closure: HashSet<_> = set.iter().copied().collect();
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
