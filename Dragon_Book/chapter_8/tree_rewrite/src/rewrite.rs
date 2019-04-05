use super::{BinOp, Cst, Node, Reg};
use crate::machine_code::{Addr, Binary, Code};

type Rule = Box<Fn(&Node) -> Option<(Node, Code)>>;

#[derive(Debug)]
pub enum RewriteError {
    Stuck(Node),
}

#[rustfmt::skip]
fn load(node: &Node) -> Option<(Node, Code)> {
    use Node::{Ind, Mem, Op};

    match node {
        Ind(box Op(
            box Node::Cst(Cst::Var(arr)), 
            BinOp::Add, 
            box Node::Reg(idx)
        )) => {
            let ld = Code::Ld(Reg::NP, Addr::Idx(*arr, *idx));
            Some((Node::Reg(Reg::NP), ld))
        }
        Ind(box Node::Reg(ptr)) => {
            let reg = if *ptr == Reg::SP {
                Reg::NP
            } else {
                *ptr
            };

            let code = Code::Ld(reg, Addr::Ref(*ptr));
            Some((Node::Reg(reg), code))
        }
        Node::Cst(cst) => Some((Node::Reg(Reg::NP), Code::Ld(Reg::NP, Addr::Cst(*cst)))),
        Mem(mem) => Some((Node::Reg(Reg::NP), Code::Ld(Reg::NP, Addr::Mem(*mem)))),
        _ => None,
    }
}

#[rustfmt::skip]
fn store(node: &Node) -> Option<(Node, Code)> {
    match node {
        Node::Assign(
            box Node::Ind(box Node::Reg(dst)), 
            box Node::Reg(src)
        ) => {
            Some((Node::End, Code::St(Addr::Ref(*dst), *src)))
        }
        Node::Assign(
            box Node::Mem(dst), 
            box Node::Reg(src)
        ) => {
            Some((Node::End, Code::St(Addr::Mem(*dst), *src)))
        }
        _ => None,
    }
}

#[rustfmt::skip]
fn op(node: &Node) -> Option<(Node, Code)> {
    match node {
        Node::Op(
            box Node::Reg(lhs),
            op,
            box Node::Ind(box Node::Op(
                box Node::Cst(Cst::Var(arr)),
                BinOp::Add,
                box Node::Reg(idx),
            )),
        ) => {
            let code = Code::Op(*op, *lhs, Addr::Reg(*lhs), Addr::Idx(*arr, *idx));
            Some((Node::Reg(*lhs), code))
        }
        Node::Op(
            box Node::Reg(lhs), 
            op, 
            box Node::Reg(rhs)
        ) => {
            let code = Code::Op(*op, *lhs, Addr::Reg(*lhs), Addr::Reg(*rhs));
            Some((Node::Reg(*lhs), code))
        }
        Node::Op(
            box Node::Reg(lhs), 
            op, 
            box Node::Cst(rhs)
        ) => {
            let code = if *op == BinOp::Add && *rhs == Cst::Lit(1) {
                Code::Inc(*lhs)
            } else {
                Code::Op(*op, *lhs, Addr::Reg(*lhs), Addr::Cst(*rhs))
            };

            Some((Node::Reg(*lhs), code))
        }
        _ => None,
    }
}

fn rules() -> Vec<Rule> {
    vec![Box::new(op), Box::new(store), Box::new(load)]
}

#[derive(Default)]
pub struct Rewriter {
    codes: Vec<Code>,
    rules: Vec<Rule>,
    next_reg: u8,
}

impl Rewriter {
    pub fn new() -> Self {
        Rewriter {
            rules: rules(),
            ..Self::default()
        }
    }

    fn apply(&mut self, tree: &mut Node, is_lvalue: bool) -> bool {
        if is_lvalue {
            match tree {
                Node::Mem(..) => return false,
                Node::Ind(box Node::Reg(..)) => return false,
                _ => (),
            }
        }

        if let Some((mut node, mut code)) = self.rules.iter().find_map(|rule| rule(tree)) {
            if node == Node::Reg(Reg::NP) {
                code = code.alloc(self.next_reg);
                node = Node::Reg(Reg::GP(self.next_reg));
                self.next_reg += 1;
            }
            *tree = node;
            self.codes.push(code);
            true
        } else {
            false
        }
    }

    fn rewrite(&mut self, tree: &mut Node, is_lvalue: bool) {
        self.apply(tree, is_lvalue);

        match tree {
            Node::Assign(dst, _) => {
                self.rewrite(dst, true);
            }
            Node::Op(lhs, _, _) => {
                self.rewrite(lhs, false);
            }
            Node::Ind(inner) => self.rewrite(inner, false),
            _ => (),
        }

        self.apply(tree, is_lvalue);

        match tree {
            Node::Assign(_, src) => {
                self.rewrite(src, false);
            }
            Node::Op(_, _, rhs) => {
                self.rewrite(rhs, false);
            }
            Node::Ind(inner) => self.rewrite(inner, false),
            _ => (),
        }

        self.apply(tree, is_lvalue);
    }

    pub fn rewrite_root(&mut self, mut root: Node) -> Result<Binary, RewriteError> {
        use std::mem;

        self.rewrite(&mut root, false);
        if !root.is_leaf() {
            Err(RewriteError::Stuck(root))
        } else {
            let codes = mem::replace(&mut self.codes, vec![]);
            Ok(Binary::new(codes))
        }
    }
}

#[test]
fn rewrite_test() {
    let expr = "a[i] = b + 1;";
    let tree = Node::parse(expr).unwrap();
    let mut rewriter = Rewriter::new();

    let binary = rewriter.rewrite_root(tree).unwrap();
    // println!("{:?}", binary);
    assert_eq!(binary.len(), 6);
}
