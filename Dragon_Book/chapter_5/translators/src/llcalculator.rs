use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub llinfix);

const LEVEL: usize = 4;

pub struct ExprNode {
    term: TermNode,
    tail: ExprdNode,
    val: i32,
}

impl Debug for ExprNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.format(0, f)
    }
}

impl ExprNode {
    fn new(term: TermNode, tail: ExprdNode) -> Self {
        ExprNode { term, tail, val: 0 }
    }

    pub fn parse<'a>(s: &'a str) -> Result<ExprNode, Box<Error + 'a>> {
        let mut node = llinfix::LParser::new().parse(s)?;
        node.attach_attrs();
        Ok(node)
    }

    fn attach_attrs(&mut self) {
        self.term.attach_attrs();
        self.tail.inh = self.term.val;
        self.tail.attach_attrs();
        self.val = self.tail.syn;
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "E.val = {}", self.val)?;
        self.term.format(indent + LEVEL, f)?;
        self.tail.format(indent + LEVEL, f)
    }
}

pub struct ExprdNode {
    inh: i32,
    syn: i32,
    expr: Exprd,
}

impl ExprdNode {
    fn cons(term: TermNode, tail: ExprdNode) -> Self {
        let expr = Exprd::Cons(term, Box::new(tail));
        ExprdNode {
            inh: 0,
            syn: 0,
            expr,
        }
    }

    fn empty() -> Self {
        ExprdNode {
            inh: 0,
            syn: 0,
            expr: Exprd::Empty,
        }
    }

    fn attach_attrs(&mut self) {
        match &mut self.expr {
            Exprd::Empty => self.syn = self.inh,
            Exprd::Cons(term, tail) => {
                term.attach_attrs();
                tail.inh = self.inh + term.val;
                tail.attach_attrs();
                self.syn = tail.syn;
            }
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "E'.inh = {}, E.syn = {}", self.inh, self.syn)?;
        if let Exprd::Cons(term, tail) = &self.expr {
            term.format(indent + LEVEL, f)?;
            tail.format(indent + LEVEL, f)?;
        }
        Ok(())
    }
}

enum Exprd {
    Empty,
    Cons(TermNode, Box<ExprdNode>),
}

pub struct TermNode {
    factor: FactorNode,
    tail: TermdNode,
    val: i32,
}

impl TermNode {
    fn new(factor: FactorNode, tail: TermdNode) -> Self {
        TermNode {
            factor,
            tail,
            val: 0,
        }
    }

    fn attach_attrs(&mut self) {
        self.factor.attach_attrs();
        self.tail.inh = self.factor.val;
        self.tail.attach_attrs();
        self.val = self.tail.syn;
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "T.val = {}", self.val)?;
        self.factor.format(indent + LEVEL, f)?;
        self.tail.format(indent + LEVEL, f)
    }
}

pub struct TermdNode {
    inh: i32,
    syn: i32,
    term: Termd,
}

impl TermdNode {
    fn cons(factor: FactorNode, tail: TermdNode) -> Self {
        let term = Termd::Cons(factor, Box::new(tail));
        TermdNode {
            inh: 0,
            syn: 0,
            term,
        }
    }

    fn empty() -> Self {
        TermdNode {
            inh: 0,
            syn: 0,
            term: Termd::Empty,
        }
    }

    fn attach_attrs(&mut self) {
        match &mut self.term {
            Termd::Empty => self.syn = self.inh,
            Termd::Cons(factor, tail) => {
                factor.attach_attrs();
                tail.inh = self.inh * factor.val;
                tail.attach_attrs();
                self.syn = tail.syn;
            }
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "T'.inh = {}, T'.syn = {}", self.inh, self.syn)?;
        if let Termd::Cons(factor, tail) = &self.term {
            factor.format(indent + LEVEL, f)?;
            tail.format(indent + LEVEL, f)?;
        }
        Ok(())
    }
}

enum Termd {
    Empty,
    Cons(FactorNode, Box<TermdNode>),
}

pub struct FactorNode {
    val: i32,
    factor: Factor,
}

impl FactorNode {
    fn paren(expr: ExprNode) -> Self {
        FactorNode {
            val: 0,
            factor: Factor::Paren(Box::new(expr)),
        }
    }

    fn lit(n: i32) -> Self {
        FactorNode {
            val: 0,
            factor: Factor::Lit(n),
        }
    }

    fn attach_attrs(&mut self) {
        match &mut self.factor {
            Factor::Lit(lit) => self.val = *lit,
            Factor::Paren(expr) => {
                expr.attach_attrs();
                self.val = expr.val;
            }
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "F.val = {}", self.val)?;
        if let Factor::Paren(expr) = &self.factor {
            expr.format(indent + LEVEL, f)?;
        }
        Ok(())
    }
}

enum Factor {
    Lit(i32),
    Paren(Box<ExprNode>),
}

fn pad(indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:width$}", "", width = indent)
}

#[test]
fn calculator_test() {
    assert_eq!(ExprNode::parse("(3 + 4) * (5 + 6) n").unwrap().val, 77);
    assert_eq!(ExprNode::parse("1 * 2 * 3 * (4 * 5) n").unwrap().val, 120);
    assert_eq!(ExprNode::parse("(9 + 8 * (7 + 6) + 5) n").unwrap().val, 118);
}
