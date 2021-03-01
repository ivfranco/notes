#![deny(missing_docs)]

//! query language with bag semantics.

/// A few pre-defined relations.
pub mod relations;

use rust_decimal::Decimal;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    io::{Cursor, Read},
    iter::{repeat, FromIterator},
};

/// atomic type inside tuples.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    /// String type with no length limit.
    String(String),
    /// Std float is not Ord.
    Number(Decimal),
}

impl Atom {
    /// Parse a token as an atomic value.
    /// Always succeeds as the token is always a valid Atom::String.
    fn parse(token: &str) -> Self {
        if let Ok(d) = token.parse::<Decimal>() {
            Atom::Number(d)
        } else if token.is_ascii() {
            Atom::String(token.to_string())
        } else {
            // otherwise it's extremely tiresome to calculate the width of the string
            // this assumption simplifies the formatter
            panic!("Non-ASCII string");
        }
    }

    fn width(&self) -> usize {
        match self {
            Atom::String(s) => s.len(),
            Atom::Number(d) => format!("{}", d).len(),
        }
    }
}

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Atom::String(s) => <String as Display>::fmt(s, f),
            Atom::Number(d) => <Decimal as Display>::fmt(d, f),
        }
    }
}

/// A tuple, a single row inside bags.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tuple<T = Atom> {
    inner: Vec<T>,
}

impl<T: Clone> Tuple<T> {
    fn project(&self, indices: &[usize]) -> Self {
        let inner = indices.iter().map(|&i| self.inner[i].clone()).collect();
        Self { inner }
    }
}

impl<'a> FromIterator<&'a str> for Tuple {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let inner = iter.into_iter().map(Atom::parse).collect();
        Self { inner }
    }
}

/// Helper structure building Bags from iterators.
struct Tuples<T = Atom> {
    inner: BTreeMap<Tuple<T>, usize>,
}

impl<T: Ord> FromIterator<(Tuple<T>, usize)> for Tuples<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Tuple<T>, usize)>,
    {
        let mut inner = BTreeMap::new();

        for (tuple, cnt) in iter.into_iter() {
            match inner.entry(tuple) {
                Entry::Occupied(mut e) => {
                    *e.get_mut() += cnt;
                }
                Entry::Vacant(e) => {
                    e.insert(cnt);
                }
            }
        }

        Self { inner }
    }
}

impl<T: Ord> FromIterator<Tuple<T>> for Tuples<T> {
    fn from_iter<I: IntoIterator<Item = Tuple<T>>>(iter: I) -> Self {
        iter.into_iter().zip(repeat(1)).collect()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Header {
    prefix: Option<String>,
    column: String,
}

impl Header {
    fn new(column: &str) -> Self {
        Self {
            prefix: None,
            column: column.to_string(),
        }
    }

    fn with_prefix(&self, prefix: &str) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
            column: self.column.clone(),
        }
    }

    fn width(&self) -> usize {
        if let Some(prefix) = self.prefix.as_ref() {
            prefix.len() + self.column.len() + 1
        } else {
            self.column.len()
        }
    }
}

impl From<&str> for Header {
    fn from(str: &str) -> Header {
        if let Some(dot) = str.find('.') {
            let (prefix, column) = str.split_at(dot);
            Self {
                prefix: Some(prefix.to_string()),
                column: column.to_string(),
            }
        } else {
            Self::new(str)
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = self.prefix.as_ref() {
            write!(f, "{}.{}", prefix, self.column)
        } else {
            write!(f, "{}", self.column)
        }
    }
}

impl Debug for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

/// A.k.a multisets.
#[derive(PartialEq)]
pub struct Bag<T = Atom> {
    /// headers of tuples.
    headers: Vec<Header>,
    /// tuples of a Bag with occurrence counts.
    tuples: BTreeMap<Tuple<T>, usize>,
}

impl Bag<Atom> {
    /// Read a space seperated csv file with headers from a reader into a Bag.
    /// No error handling for now, the only expected input are relations from the textbook.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Box<dyn Error>> {
        let mut csv = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .from_reader(reader);

        let headers = csv.headers()?.iter().map(Header::new).collect();

        let tuples = csv
            .into_records()
            .map(|r| r.map(|r| r.iter().collect::<Tuple>()))
            .collect::<csv::Result<Tuples>>()?
            .inner;

        Ok(Self { headers, tuples })
    }

    /// Read a space seperated csv file with headers from a string into a Bag.
    /// No error handling for now, the only expected input are relations from the textbook.
    pub fn from_csv(str: &str) -> Result<Self, Box<dyn Error>> {
        Self::from_reader(Cursor::new(str))
    }
}

impl<T> Bag<T>
where
    T: Clone + Ord,
{
    /// The number of tuples in the bag.
    pub fn len(&self) -> usize {
        self.tuples.values().sum()
    }

    /// Query the occurrence counter of a tuple.
    pub fn get(&self, tuple: &Tuple<T>) -> usize {
        self.tuples.get(tuple).copied().unwrap_or(0)
    }

    /// The number of tuples in the bag.
    /// The internal HashMap may not be empty but all occurrence counts are 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return a new bag with headers prefixed.
    pub fn prefix(&self, prefix: &str) -> Self {
        let headers = self.headers.iter().map(|h| h.with_prefix(prefix)).collect();
        Self {
            headers,
            tuples: self.tuples.clone(),
        }
    }

    /// set all occurrence count to 1, pretends the bag is a set
    pub fn dedup(&self) -> Self {
        let tuples = self.tuples().map(|(t, _)| (t, 1)).collect();
        Self {
            headers: self.headers.clone(),
            tuples,
        }
    }

    fn tuples(&self) -> impl Iterator<Item = (Tuple<T>, usize)> + '_ {
        self.tuples.iter().map(|(t, c)| (Tuple::clone(t), *c))
    }

    /// Projection of a bag to its columns.
    /// Panics if the projection headers are not headers of the bag.
    pub fn project<'a, A>(&self, headers: A) -> Self
    where
        A: AsRef<[&'a str]>,
    {
        let headers: Vec<_> = headers.as_ref().iter().map(|&s| s.into()).collect();
        let indices: Vec<_> = headers
            .iter()
            .map(|h| self.headers.iter().position(|s| s == h).unwrap())
            .collect();
        let tuples = self
            .tuples
            .iter()
            .map(|(t, c)| (t.project(&indices), *c))
            .collect::<Tuples<T>>()
            .inner;

        Self { headers, tuples }
    }

    /// Union of bags.
    /// Panics if the headers of the two bags do not match.
    pub fn union(&self, other: &Self) -> Self {
        assert_eq!(self.headers, other.headers);

        let tuples = self
            .tuples()
            .chain(other.tuples())
            .collect::<Tuples<T>>()
            .inner;

        Self {
            headers: self.headers.clone(),
            tuples,
        }
    }

    /// Intersection of bags.
    /// Panics if the headers of the two bags do not match.
    pub fn intersection(&self, other: &Self) -> Self {
        assert_eq!(self.headers, other.headers);

        let tuples = self
            .tuples
            .iter()
            .filter_map(|(t, c0)| other.tuples.get(t).map(|c1| (t.clone(), *c0.min(c1))))
            .collect::<Tuples<T>>()
            .inner;

        Self {
            headers: self.headers.clone(),
            tuples,
        }
    }

    /// difference of bags.
    /// Panics if the headers of the two bags do not match.
    pub fn difference(&self, rhs: &Self) -> Self {
        assert_eq!(self.headers, rhs.headers);

        let tuples = self
            .tuples()
            .filter_map(|(t, c0)| {
                let c1 = rhs.get(&t);
                if c0 > c1 {
                    Some((t, c0 - c1))
                } else {
                    None
                }
            })
            .collect::<Tuples<T>>()
            .inner;

        Self {
            headers: self.headers.clone(),
            tuples,
        }
    }

    /// A higher order function similar to fold.
    /// Panics when the bag has more than one column.
    pub fn aggregate<F, R>(&self, init: R, mut f: F) -> R
    where
        F: FnMut(R, &T, usize) -> R,
    {
        assert_eq!(self.headers.len(), 1);

        self.tuples
            .iter()
            .fold(init, |state, (t, c)| f(state, &t.inner[0], *c))
    }
}

fn sep_by_vert_bar<I>(iter: I, widths: &[usize], f: &mut Formatter) -> fmt::Result
where
    I: IntoIterator,
    I::Item: Display,
{
    let mut first = true;

    for (item, width) in iter.into_iter().zip(widths) {
        if !first {
            write!(f, "|")?;
        }
        first = false;
        write!(f, "{:<1$}", item, width)?;
    }

    Ok(())
}

impl Display for Bag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // maximum lengths of atomic values in each column
        let mut widths: Vec<_> = self.headers.iter().map(|h| h.width()).collect();

        for tuple in self.tuples.keys() {
            for (i, atom) in tuple.inner.iter().enumerate() {
                widths[i] = widths[i].max(atom.width());
            }
        }

        sep_by_vert_bar(&self.headers, &widths, f)?;
        writeln!(f)?;

        // total width with vertical bars between columns
        let total_width = widths.iter().sum::<usize>() + widths.len() - 1;

        writeln!(f, "{:-<1$}", "", total_width)?;

        for (tuple, cnt) in self.tuples.iter() {
            for _ in 0..*cnt {
                sep_by_vert_bar(&tuple.inner, &widths, f)?;
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Debug for Bag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn from_reader_test() -> Result<(), Box<dyn Error>> {
        let bag = Bag::from_reader(File::open(relations::RELATIONS.join("PC.csv"))?)?;
        let headers = ["model", "speed", "ram", "hd", "price"]
            .iter()
            .map(|&s| s.into())
            .collect::<Vec<_>>();
        assert_eq!(bag.headers, headers);
        assert_eq!(bag.len(), 13);
        Ok(())
    }

    #[test]
    fn project_test() {
        const MODEL_AND_SPEED: &str = "model speed
1001 2.66
1002 2.10
1003 1.42
1004 2.80
1005 3.20
1006 3.20
1007 2.20
1008 2.20
1009 2.00
1010 2.80
1011 1.86
1012 2.80
1013 3.06";

        assert_eq!(
            Bag::from_csv(MODEL_AND_SPEED).unwrap(),
            relations::PC.project(["model", "speed"]),
        )
    }

    #[test]
    fn aggregation_test() {
        fn max(bag: &Bag) -> Option<Decimal> {
            bag.aggregate(None, |m, a, _cnt| {
                if let Atom::Number(d) = a {
                    Some(m.unwrap_or(*d).max(*d))
                } else {
                    m
                }
            })
        }

        assert_eq!(
            max(&relations::PC.project(["ram"])),
            Some(Decimal::new(2048, 0))
        );
    }

    #[test]
    fn set_operations() {
        const LHS: &str = "A B
1 2
1 2
1 2
3 4";

        const RHS: &str = "A B
1 2
3 4
3 4
5 6";

        let lhs = Bag::from_csv(LHS).unwrap();
        let rhs = Bag::from_csv(RHS).unwrap();

        const UNION: &str = "A B
1 2
1 2
1 2
1 2
3 4
3 4
3 4
5 6";

        assert_eq!(lhs.union(&rhs), Bag::from_csv(UNION).unwrap());

        const INTERSECTION: &str = "A B
1 2
3 4";

        assert_eq!(lhs.intersection(&rhs), Bag::from_csv(INTERSECTION).unwrap());

        const DIFFERENCE: &str = "A B
1 2
1 2";

        assert_eq!(lhs.difference(&rhs), Bag::from_csv(DIFFERENCE).unwrap());
    }
}
