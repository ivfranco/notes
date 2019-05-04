mod affine;

use affine::{Affine, Coeff, Var};

//  (a, b) represents a <= b
type Constraint<'a> = (&'a [Coeff], &'a [Coeff]);

struct Matrix {
    slots: Vec<Coeff>,
    columns: usize,
}

impl Matrix {
    fn new(columns: usize) -> Self {
        Matrix {
            slots: vec![],
            columns,
        }
    }

    fn push_row(&mut self, row: &[Coeff]) {
        assert!(row.len() <= self.columns);
        self.slots.extend(row);
        for _ in 0..self.columns - row.len() {
            self.slots.push(0);
        }
    }

    fn rows(&self) -> impl Iterator<Item = &[Coeff]> {
        self.slots.chunks_exact(self.columns)
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "[")?;
        for row in self.rows() {
            write!(f, "{:4}", "")?;
            f.debug_list().entries(row).finish()?;
            writeln!(f, ",")?;
        }
        writeln!(f, "]")
    }
}

pub struct Triple {
    matrix: Matrix,
    vars: Vec<Var>,
    constants: Vec<Coeff>,
}

impl Triple {
    pub fn new(vars: &[Var], constraints: &[Constraint]) -> Self {
        assert!(constraints
            .iter()
            .all(|(less, more)| less.len() <= vars.len() + 1 && more.len() <= vars.len() + 1));

        let mut triple = Triple {
            matrix: Matrix::new(vars.len()),
            vars: vars.to_vec(),
            constants: vec![],
        };

        for (less, more) in constraints {
            triple.add_constraint(&Affine::new(less.to_vec()), &Affine::new(more.to_vec()));
        }

        triple
    }

    fn add_constraint(&mut self, less: &Affine, more: &Affine) {
        let sub = more - less;
        self.matrix.push_row(sub.non_constants());
        self.constants.push(sub.constant());
    }

    fn get_var(&self, var: Var) -> usize {
        self.vars
            .iter()
            .position(|v| *v == var)
            .expect("Get var: not a variable of the triple")
            + 1
    }

    pub fn eliminate(&self, var: Var) -> Option<Triple> {
        let var_idx = self.get_var(var);

        let mut vars = self.vars.clone();
        vars.remove(var_idx - 1);
        let mut triple = Triple::new(&vars, &[]);

        for affine in self.affines().filter(|affine| !affine.uses(var_idx)) {
            let (_, remain) = affine.eliminate_var(var_idx);
            triple.add_constraint(&Affine::new_zero(), &remain);
        }

        for (c0, lower) in self.lower_bounds(var_idx) {
            for (c1, upper) in self.upper_bounds(var_idx) {
                let sub = &(&upper * c0) - &(&lower * c1);
                if sub.is_constant() {
                    if sub.constant() < 0 {
                        return None;
                    }
                } else {
                    triple.add_constraint(&Affine::new_zero(), &sub);
                }
            }
        }

        Some(triple)
    }

    pub fn report_constraints(mut self, vars: &[Var]) {
        for var in vars {
            let var_idx = self.get_var(*var);
            let mut vars = self.vars.clone();
            vars.remove(var_idx - 1);
            for (coeff, lower) in self.lower_bounds(var_idx) {
                if coeff == 1 {
                    println!("{} <= {}", lower.format(&vars), var);
                } else {
                    println!("{} <= {}{}", lower.format(&vars), coeff, var);
                }
            }
            for (coeff, upper) in self.upper_bounds(var_idx) {
                if coeff == 1 {
                    println!("{} <= {}", var, upper.format(&vars));
                } else {
                    println!("{}{} <= {}", coeff, var, upper.format(&vars));
                }
            }

            self = self.eliminate(*var).unwrap();
        }
    }

    fn affines<'a>(&'a self) -> impl Iterator<Item = Affine> + 'a {
        self.matrix.rows().enumerate().map(move |(i, row)| {
            let constant = self.constants[i];
            let mut coeffs = vec![constant];
            coeffs.extend(row);
            Affine::new(coeffs)
        })
    }

    fn lower_bounds(&self, var_idx: usize) -> Vec<(Coeff, Affine)> {
        self.affines()
            .filter(|affine| affine.get(var_idx) > 0)
            .map(|affine| {
                let (coeff, remain) = affine.eliminate_var(var_idx);
                (coeff, -&remain)
            })
            .collect()
    }

    fn upper_bounds(&self, var_idx: usize) -> Vec<(Coeff, Affine)> {
        self.affines()
            .filter(|affine| affine.get(var_idx) < 0)
            .map(|affine| {
                let (coeff, remain) = affine.eliminate_var(var_idx);
                (-coeff, remain)
            })
            .collect()
    }
}

impl std::fmt::Debug for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Matrix B: {:?}", self.matrix)?;
        writeln!(f, "variables i: {:?}T", self.vars)?;
        writeln!(f, "constants b: {:?}T", self.constants)
    }
}

#[cfg(test)]
fn figure_11_11() -> Triple {
    Triple::new(
        &['i', 'j'],
        &[
            (&[0], &[0, 1]),
            (&[0, 1], &[5]),
            (&[0, 1], &[0, 0, 1]),
            (&[0, 0, 1], &[7]),
        ],
    )
}

#[test]
fn triple_test() {
    let triple = figure_11_11();
    assert_eq!(triple.matrix.slots, &[1, 0, -1, 0, -1, 1, 0, -1]);
    assert_eq!(triple.constants, &[0, 5, 0, 7]);
}

#[test]
fn eliminate_test() {
    let triple = figure_11_11();
    let _eliminated = triple.eliminate('i').unwrap();
}

#[test]
fn report_constraints_test() {
    let triple = figure_11_11();
    triple.report_constraints(&['i', 'j']);
}
