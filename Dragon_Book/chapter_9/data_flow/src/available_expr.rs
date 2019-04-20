use crate::{Block, BlockID, Expr, Program};
use std::collections::HashSet;
use std::fmt::{self, Formatter};

struct GenKill<'a> {
    gen: HashSet<Expr<'a>>,
    kill: HashSet<&'a str>,
}

impl<'a> GenKill<'a> {
    fn new(block: &'a Block) -> Self {
        let mut gen = HashSet::new();
        let mut kill = HashSet::new();

        for (_, stmt) in block.stmts_indices() {
            gen.extend(stmt.as_expr());
            if let Some(def) = stmt.def() {
                gen.retain(|expr| !expr.uses(def));
                kill.insert(def);
            }
        }

        GenKill { gen, kill }
    }

    fn transfer(&self, in_set: &HashSet<Expr<'a>>) -> HashSet<Expr<'a>> {
        in_set
            .iter()
            .filter(|expr| self.kill.iter().all(|var| !expr.uses(var)))
            .chain(&self.gen)
            .cloned()
            .collect()
    }
}

struct AvailableExpression<'a> {
    in_set: HashSet<Expr<'a>>,
    out_set: HashSet<Expr<'a>>,
    gen_kill: GenKill<'a>,
}

impl<'a> AvailableExpression<'a> {
    fn new(block_id: BlockID, program: &'a Program) -> Self {
        AvailableExpression {
            in_set: HashSet::new(),
            out_set: program.exprs().collect(),
            gen_kill: GenKill::new(program.get_block(block_id).unwrap()),
        }
    }

    fn update(&mut self) -> bool {
        let new_out = self.gen_kill.transfer(&self.in_set);
        let changed = self.out_set != new_out;
        self.out_set = new_out;
        changed
    }

    fn format(&self, id: &str, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "attributes of {}:", id)?;
        writeln!(f, "    IN: {:?}", self.in_set)?;
        writeln!(f, "    OUT: {:?}", self.out_set)?;
        writeln!(f, "    e_gen: {:?}", self.gen_kill.gen)?;
        writeln!(f, "    e_kill: {:?}", self.gen_kill.kill)
    }
}

pub struct AvailableExpressions<'a> {
    attrs: Vec<AvailableExpression<'a>>,
}

impl<'a> std::fmt::Debug for AvailableExpressions<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, attr) in self.attrs.iter().enumerate() {
            let id = if i == 0 {
                "ENTRY".to_string()
            } else if i + 1 == self.attrs.len() {
                "EXIT".to_string()
            } else {
                format!("B{}", i)
            };

            attr.format(&id, f)?;
        }
        Ok(())
    }
}

fn meet<'a>(
    block_id: BlockID,
    program: &'a Program,
    attrs: &[AvailableExpression<'a>],
) -> HashSet<Expr<'a>> {
    program
        .predecessors(block_id)
        .map(|(p, _)| &attrs[p].out_set)
        .fold(HashSet::new(), |set, out_p| {
            if set.is_empty() {
                out_p.clone()
            } else {
                &set & out_p
            }
        })
}

pub fn available_expressions(program: &Program) -> AvailableExpressions<'_> {
    let mut attrs: Vec<_> = program
        .blocks()
        .enumerate()
        .map(|(i, _)| AvailableExpression::new(i, program))
        .collect();
    attrs[program.entry()].out_set.clear();
    let mut updated = true;

    while updated {
        updated = false;
        for block_id in 0..program.len() {
            attrs[block_id].in_set = meet(block_id, program, &attrs);
            updated = attrs[block_id].update() || updated;
        }
    }

    AvailableExpressions { attrs }
}

#[test]
fn available_expressions_test() {
    use crate::{BinOp, RValue};

    let block = Block::parse(
        0,
        "a = b + c
b = a - d
c = b + c
d = a - d",
    );

    let gen_kill = GenKill::new(&block);
    assert_eq!(gen_kill.gen, HashSet::new());
    assert_eq!(
        gen_kill.kill,
        vec!["a", "b", "c", "d"].into_iter().collect()
    );

    let program = Program::new(
        vec![
            Block::entry(),
            Block::parse(1, "t1 = 4 * i"),
            Block::parse(2, "i = 1\nt1 = 4 * i"),
            Block::parse(3, "t2 = 4 * i"),
            Block::exit(),
        ],
        &[(0, 1), (1, 2), (1, 3), (2, 3), (3, 4)],
    );

    let avs = available_expressions(&program);
    // println!("{:?}", avs);
    let expr = Expr {
        lhs: &RValue::Lit(4),
        op: BinOp::Mul,
        rhs: &RValue::Var("i".to_string()),
    };
    assert_eq!(
        avs.attrs[3].in_set,
        Some(expr.clone()).into_iter().collect()
    );
    assert_eq!(avs.attrs[3].out_set, Some(expr).into_iter().collect());
}
