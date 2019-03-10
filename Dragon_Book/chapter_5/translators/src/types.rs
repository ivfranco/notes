use crate::utils::pad;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};

lalrpop_mod!(pub decl);

const LEVEL: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ty {
    Int,
    Float,
    Undefined,
}

impl Ty {
    fn format(self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "T.type = {:?}", self)
    }
}

pub struct DeclNode {
    ty: Ty,
    vars: VarsNode,
}

impl DeclNode {
    pub fn parse<'a>(s: &'a str) -> Result<Self, Box<Error + 'a>> {
        let mut node = decl::DParser::new().parse(s)?;
        node.attach_attrs();
        Ok(node)
    }

    fn attach_attrs(&mut self) {
        self.vars.inh = self.ty;
        self.vars.attach_attrs();
    }

    pub fn install(&self, ctx: &mut HashMap<String, Ty>) {
        self.vars.install(ctx);
    }
}

impl Debug for DeclNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "D (with no attributes)")?;
        self.ty.format(LEVEL, f)?;
        self.vars.format(LEVEL, f)
    }
}

pub struct VarsNode {
    inh: Ty,
    vars: Vars,
}

impl VarsNode {
    pub fn id(id: String) -> Self {
        VarsNode {
            inh: Ty::Undefined,
            vars: Vars::Id(id),
        }
    }

    pub fn cons(node: VarsNode, id: String) -> Self {
        VarsNode {
            inh: Ty::Undefined,
            vars: Vars::Cons(Box::new(node), id),
        }
    }

    fn attach_attrs(&mut self) {
        if let Vars::Cons(init, _) = &mut self.vars {
            init.inh = self.inh;
            init.attach_attrs();
        }
    }

    fn install(&self, ctx: &mut HashMap<String, Ty>) {
        match &self.vars {
            Vars::Id(id) => {
                ctx.insert(id.clone(), self.inh);
            }
            Vars::Cons(init, id) => {
                ctx.insert(id.clone(), self.inh);
                init.install(ctx);
            }
        }
    }

    fn format(&self, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
        pad(indent, f)?;
        writeln!(f, "L.inh = {:?}", self.inh)?;
        match &self.vars {
            Vars::Id(id) => format_id(id, indent + LEVEL, f),
            Vars::Cons(init, id) => {
                init.format(indent + LEVEL, f)?;
                format_id(id, indent + LEVEL, f)
            }
        }
    }
}

fn format_id(id: &str, indent: usize, f: &mut Formatter) -> Result<(), fmt::Error> {
    pad(indent, f)?;
    writeln!(f, "id.entry = {}", id)
}

enum Vars {
    Id(String),
    Cons(Box<VarsNode>, String),
}

#[test]
fn install_test() {
    let node = DeclNode::parse("float w, x, y, z").unwrap();
    let mut ctx = HashMap::new();
    node.install(&mut ctx);

    assert_eq!(ctx.get("w"), Some(&Ty::Float));
    assert_eq!(ctx.get("x"), Some(&Ty::Float));
    assert_eq!(ctx.get("y"), Some(&Ty::Float));
    assert_eq!(ctx.get("z"), Some(&Ty::Float));
    assert_eq!(ctx.len(), 4);
}
