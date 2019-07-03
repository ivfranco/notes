use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    ops::{BitAnd, BitOr, Not, Shr},
    rc::Rc,
};

use itertools::Itertools;

pub mod build {
    use super::*;

    pub fn var(v: Var) -> Term {
        Variable(v)
    }

    pub fn cst<T>(c: T) -> Term
    where
        T: Into<Cst>,
    {
        Constant(c.into())
    }

    pub fn func<T>(name: T, args: Vec<Term>) -> Term
    where
        T: Into<Func>,
    {
        Function(name.into(), args)
    }

    pub fn pred<T>(name: T, args: Vec<Term>) -> Sentence
    where
        T: Into<Pred>,
    {
        Predicate(name.into(), args)
    }

    pub fn not(s: Sentence) -> Sentence {
        Not(Rc::new(s))
    }

    pub fn and(lhs: Sentence, rhs: Sentence) -> Sentence {
        And(Rc::new(lhs), Rc::new(rhs))
    }

    pub fn or(lhs: Sentence, rhs: Sentence) -> Sentence {
        Or(Rc::new(lhs), Rc::new(rhs))
    }

    pub fn imply(lhs: Sentence, rhs: Sentence) -> Sentence {
        Imply(Rc::new(lhs), Rc::new(rhs))
    }

    pub fn iff(lhs: Sentence, rhs: Sentence) -> Sentence {
        Iff(Rc::new(lhs), Rc::new(rhs))
    }

    pub fn exist(v: Var, s: Sentence) -> Sentence {
        Quantified(Existential(v), Rc::new(s))
    }

    pub fn forall(v: Var, s: Sentence) -> Sentence {
        Quantified(Univeral(v), Rc::new(s))
    }

    pub fn equal(lhs: Term, rhs: Term) -> Sentence {
        Equal(lhs, rhs)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Quantifier {
    Univeral(Var),
    Existential(Var),
}

use Quantifier::*;

impl std::fmt::Debug for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Univeral(v) => write!(f, "∀{}", v),
            Existential(v) => write!(f, "∃{}", v),
        }
    }
}

// start with upper caes letter
pub type Cst = String;
// lower case letters
pub type Var = char;
// start with upper case letter
pub type Func = String;
// start with upper case letter
pub type Pred = String;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Constant(Cst),
    Variable(Var),
    Function(Func, Vec<Self>),
}

use self::Term::*;

impl Term {
    pub fn parse(s: &str) -> Self {
        TermParser::new(s).parse()
    }

    fn occur_check(&self, var: Var) -> bool {
        match self {
            Variable(v) => *v == var,
            Constant(..) => false,
            Function(_, args) => args.iter().any(|arg| arg.occur_check(var)),
        }
    }

    pub fn subst(&self, unifier: &Unifier) -> Term {
        match self {
            Variable(v) => {
                if let Some(t) = unifier.get(v) {
                    //  for substitutions e.g. {x/F(y), y/C}, a single application cannot reduce x to ground term
                    t.subst(unifier)
                } else {
                    self.clone()
                }
            }
            Function(name, args) => Function(
                name.to_string(),
                args.iter().map(|arg| arg.subst(unifier)).collect(),
            ),
            _ => self.clone(),
        }
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Constant(c) => write!(f, "{}", c),
            Variable(v) => write!(f, "{}", v),
            Function(name, args) => {
                write!(f, "{}", name)?;
                write!(f, "({:?})", args.iter().format(", "))?;
                Ok(())
            }
        }
    }
}

struct TermParser<'a> {
    input: &'a str,
}

impl<'a> TermParser<'a> {
    fn new(input: &'a str) -> Self {
        TermParser { input }
    }

    fn consume(&mut self, token: &str) {
        assert!(self.input.starts_with(token));
        self.input = &self.input[token.len()..];
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().next()
    }

    fn token(&mut self) -> &'a str {
        let i = self
            .input
            .find(|c: char| !c.is_ascii_alphabetic())
            .unwrap_or_else(|| self.input.len());
        let (token, remain) = self.input.split_at(i);
        self.input = remain;
        token
    }

    fn parse(&mut self) -> Term {
        use build::*;

        match self.token() {
            token if token.starts_with(|c: char| c.is_ascii_lowercase()) => var(token
                .chars()
                .next()
                .expect("TermParser::parse: empty token")),
            token => {
                if self.input.starts_with('(') {
                    let args = self.parse_args();
                    func(token, args)
                } else {
                    cst(token)
                }
            }
        }
    }

    fn parse_args(&mut self) -> Vec<Term> {
        self.consume("(");
        let mut args = vec![];
        loop {
            args.push(self.parse());
            if self.peek() == Some(')') {
                break;
            } else {
                self.consume(",");
            }
        }
        self.consume(")");
        args
    }
}

#[derive(Clone, PartialEq)]
pub enum Sentence {
    Predicate(Pred, Vec<Term>),
    Not(Rc<Self>),
    Quantified(Quantifier, Rc<Self>),
    And(Rc<Self>, Rc<Self>),
    Or(Rc<Self>, Rc<Self>),
    Imply(Rc<Self>, Rc<Self>),
    Iff(Rc<Self>, Rc<Self>),
    Equal(Term, Term),
}

use Sentence::*;

impl Sentence {
    fn precedence(&self) -> Precedence {
        match self {
            Predicate(..) => Precedence::Predicate,
            Not(..) => Precedence::Not,
            Quantified(..) => Precedence::Quantified,
            And(..) | Or(..) => Precedence::AndOr,
            Imply(..) | Iff(..) => Precedence::IffImp,
            Equal(..) => Precedence::Equal,
        }
    }

    fn format(&self, parent: Precedence, f: &mut Formatter) -> fmt::Result {
        let binary = |lhs: &Self, rhs: &Self, op: &str, f: &mut Formatter| -> fmt::Result {
            lhs.format(self.precedence(), f)?;
            write!(f, " {} ", op)?;
            rhs.format(self.precedence(), f)?;
            Ok(())
        };

        if self.precedence() <= parent {
            write!(f, "(")?;
        }

        match self {
            Predicate(name, args) => {
                write!(f, "{}", name)?;
                write!(f, "({:?})", args.iter().format(", "))?;
            }
            Not(s) => {
                write!(f, "~")?;
                s.format(self.precedence(), f)?;
            }
            And(lhs, rhs) => binary(lhs, rhs, "&", f)?,
            Or(lhs, rhs) => binary(lhs, rhs, "|", f)?,
            Imply(lhs, rhs) => binary(lhs, rhs, "=>", f)?,
            Iff(lhs, rhs) => binary(lhs, rhs, "<=>", f)?,
            Equal(lhs, rhs) => write!(f, "{:?} = {:?}", lhs, rhs)?,
            Quantified(q, s) => {
                write!(f, "{:?} ", q)?;
                s.format(self.precedence(), f)?;
            }
        }

        if self.precedence() <= parent {
            write!(f, ")")?;
        }

        Ok(())
    }

    fn map_terms<F>(&self, f: &mut F) -> Self
    where
        F: FnMut(&Term) -> Term,
    {
        use build::*;

        match self {
            Predicate(name, args) => Predicate(name.to_string(), args.iter().map(f).collect()),
            Not(s) => not(s.map_terms(f)),
            And(lhs, rhs) => and(lhs.map_terms(f), rhs.map_terms(f)),
            Or(lhs, rhs) => or(lhs.map_terms(f), rhs.map_terms(f)),
            Imply(lhs, rhs) => imply(lhs.map_terms(f), rhs.map_terms(f)),
            Iff(lhs, rhs) => iff(lhs.map_terms(f), rhs.map_terms(f)),
            Quantified(q, g) => Quantified(*q, Rc::new(g.map_terms(f))),
            Equal(lhs, rhs) => equal(f(lhs), f(rhs)),
        }
    }

    pub fn subst(&self, unifier: &Unifier) -> Self {
        self.map_terms(&mut |t: &Term| t.subst(unifier))
    }
}

impl BitAnd for Sentence {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        build::and(self, rhs)
    }
}

impl BitOr for Sentence {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        build::or(self, rhs)
    }
}

impl Shr for Sentence {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        build::imply(self, rhs)
    }
}

impl Not for Sentence {
    type Output = Self;

    fn not(self) -> Self::Output {
        build::not(self)
    }
}

impl std::fmt::Debug for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.format(Precedence::Top, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Precedence {
    Top,
    Quantified,
    IffImp,
    AndOr,
    Equal,
    Not,
    Predicate,
}

pub type Unifier = HashMap<Var, Term>;

pub fn unify(x: &Sentence, y: &Sentence) -> Option<Unifier> {
    unify_sentence(x, y, Unifier::new())
}

fn unify_sentence(x: &Sentence, y: &Sentence, unifier: Unifier) -> Option<Unifier> {
    match (x, y) {
        _ if x == y => Some(unifier),
        (Predicate(p0, args0), Predicate(p1, args1)) if p0 == p1 => args0
            .iter()
            .zip(args1.iter())
            .try_fold(unifier, |unifier, (arg0, arg1)| {
                unify_term(arg0, arg1, unifier)
            }),
        (Not(s0), Not(s1)) => unify_sentence(s0, s1, unifier),
        (And(l0, r0), And(l1, r1))
        | (Or(l0, r0), Or(l1, r1))
        | (Imply(l0, r0), Imply(l1, r1))
        | (Iff(l0, r0), Iff(l1, r1)) => {
            let left_unifier = unify_sentence(l0, l1, unifier)?;
            unify_sentence(r0, r1, left_unifier)
        }
        (Equal(l0, r0), Equal(l1, r1)) => {
            let left_unifier = unify_term(l0, l1, unifier)?;
            unify_term(r0, r1, left_unifier)
        }
        (Quantified(..), _) | (_, Quantified(..)) => {
            unreachable!("unify_sentence: Only ground sentences may be unified")
        }
        _ => None,
    }
}

pub fn unify_term(x: &Term, y: &Term, unifier: Unifier) -> Option<Unifier> {
    match (x, y) {
        _ if x == y => Some(unifier),
        (Variable(v), _) => unify_var(*v, &y, unifier),
        (_, Variable(v)) => unify_var(*v, &x, unifier),
        (Function(n0, args0), Function(n1, args1)) if n0 == n1 => args0
            .iter()
            .zip(args1.iter())
            .try_fold(unifier, |unifier, (t0, t1)| unify_term(t0, t1, unifier)),
        _ => None,
    }
}

fn unify_var(var: Var, x: &Term, mut unifier: Unifier) -> Option<Unifier> {
    if let Some(val) = unifier.get(&var).cloned() {
        return unify_term(&val, &x.clone(), unifier);
    }

    if let Variable(v) = x {
        if let Some(val) = unifier.get(v) {
            return unify_term(&build::var(var), &val.clone(), unifier);
        }
    }

    if x.occur_check(var) {
        return None;
    }

    unifier.insert(var, x.clone());
    Some(unifier)
}

#[test]
fn format_test() {
    use build::*;

    let s = forall(
        'x',
        (pred("King", vec![var('x')]) & pred("Greedy", vec![var('x')]))
            >> pred("Evil", vec![var('x')]),
    );
    assert_eq!(format!("{:?}", s), "∀x King(x) & Greedy(x) => Evil(x)");

    let s = exist(
        'x',
        pred("Parent", vec![var('x'), cst("John")])
            & equal(var('x'), func("Polymer", vec![cst("Kydd"), cst("Smith")])),
    );

    assert_eq!(
        format!("{:?}", s),
        "∃x Parent(x, John) & x = Polymer(Kydd, Smith)"
    );
}

#[test]
fn sentence_unify_test() {
    use build::*;

    let s0 = pred("Knows", vec![cst("John"), var('x')]);
    let s1 = pred("Knows", vec![var('y'), cst("Elizabeth")]);

    let unifier = unify(&s0, &s1).expect("unify_test: None returned");

    assert_eq!(s0.subst(&unifier), s1.subst(&unifier));
}

#[test]
fn term_unify_test() {
    for (s0, s1) in &[
        ("P(A,B,B)", "P(x,y,z)"),
        ("Q(y,G(A,B))", "Q(G(x,x),y)"),
        ("Older(Father(y),y)", "Older(Father(x),John)"),
        ("Knows(Father(y),y)", "Knows(x,x)"),
    ] {
        let t0 = Term::parse(s0);
        let t1 = Term::parse(s1);
        let unifier = unify_term(&t0, &t1, Unifier::new());
        if let Some(unifier) = unifier {
            assert_eq!(t0.subst(&unifier), t1.subst(&unifier));
        }
    }
}
