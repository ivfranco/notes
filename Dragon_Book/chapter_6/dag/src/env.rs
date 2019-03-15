use crate::utils::pad;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub ty);

const LEVEL: usize = 4;

#[derive(PartialEq)]
pub struct Env {
    map: HashMap<String, (Ty, usize)>,
}

impl Env {
    pub fn new(decls: Vec<(Ty, String)>) -> Self {
        let mut offset = 0;
        let mut map = HashMap::new();

        #[allow(clippy::map_entry)]
        for (ty, var) in decls {
            let width = ty.width();
            if map.contains_key(&var) {
                panic!("Error: Redeclared variable {}", var);
            } else {
                map.insert(var, (ty, offset));
            }
            offset += width;
        }

        Env { map }
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        ty::PParser::new().parse(s).map_err(Box::from)
    }

    fn width(&self) -> usize {
        self.map.values().map(|(ty, _)| ty.width()).sum()
    }

    fn format(&self, indent: usize, offset: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut decls: Vec<_> = self.map.iter().collect();
        decls.sort_by_key(|(_, (_, off))| *off);

        for (id, (ty, off)) in decls {
            pad(indent, f)?;
            writeln!(f, "{}: {:?}, offset: {}", id, ty, off + offset)?;
            if let Ty::Record(env) = ty {
                env.format(indent + LEVEL, offset + off, f)?;
            }
        }

        Ok(())
    }
}

impl Debug for Env {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.format(0, 0, f)
    }
}

#[derive(PartialEq)]
pub enum Ty {
    Int,
    Float,
    Array(usize, Box<Ty>),
    Record(Env),
}

impl Ty {
    pub fn array(ty: Ty, dims: Vec<usize>) -> Self {
        dims.into_iter()
            .rev()
            .fold(ty, |inner, dim| Ty::Array(dim, Box::new(inner)))
    }

    pub fn record(env: Env) -> Self {
        Ty::Record(env)
    }

    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        ty::TParser::new().parse(s).map_err(Box::from)
    }

    fn width(&self) -> usize {
        match self {
            Ty::Int => 4,
            Ty::Float => 8,
            Ty::Array(dim, inner) => inner.width() * dim,
            Ty::Record(env) => env.width(),
        }
    }
}

impl Debug for Ty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Ty::Int => write!(f, "int"),
            Ty::Float => write!(f, "float"),
            Ty::Array(dim, inner) => write!(f, "{:?} x {}", inner, dim),
            Ty::Record(..) => write!(f, "record {{..}}"),
        }
    }
}

#[test]
fn type_parse_test() {
    assert_eq!(
        Ty::parse("int [2][3]").unwrap(),
        Ty::Array(2, Box::new(Ty::Array(3, Box::new(Ty::Int))))
    );
}
