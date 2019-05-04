use std::cmp;
use std::ops::{Mul, Neg, Sub};

pub type Coeff = i32;
pub type Var = char;

#[derive(Clone)]
pub struct Affine {
    coeffs: Vec<Coeff>,
}

#[allow(clippy::len_without_is_empty)]
impl Affine {
    pub fn new(coeffs: Vec<Coeff>) -> Self {
        if coeffs.is_empty() {
            Affine::new_zero()
        } else {
            let mut affine = Affine { coeffs };
            affine.simplify();
            affine
        }
    }

    pub fn new_zero() -> Self {
        Affine { coeffs: vec![0] }
    }

    pub fn constant(&self) -> Coeff {
        *self.coeffs.get(0).unwrap_or(&0)
    }

    pub fn is_constant(&self) -> bool {
        self.coeffs[1..].iter().all(|coeff| *coeff == 0)
    }

    pub fn is_zero(&self) -> bool {
        self.is_constant() && self.constant() == 0
    }

    pub fn len(&self) -> usize {
        self.coeffs.len()
    }

    pub fn var_len(&self) -> usize {
        self.len() - 1
    }

    pub fn non_constants(&self) -> &[Coeff] {
        &self.coeffs[1..]
    }

    fn simplify(&mut self) {
        while let Some(0) = self.coeffs.last() {
            if self.len() == 1 {
                break;
            }
            self.coeffs.pop();
        }
    }

    pub fn get(&self, var_idx: usize) -> Coeff {
        *self.coeffs.get(var_idx).unwrap_or(&0)
    }

    pub fn uses(&self, var_idx: usize) -> bool {
        self.get(var_idx) != 0
    }

    pub fn eliminate_var(&self, var_idx: usize) -> (Coeff, Affine) {
        let mut remain = self.coeffs.clone();
        let coeff = if var_idx < remain.len() {
            remain.remove(var_idx)
        } else {
            0
        };

        (coeff, Affine::new(remain))
    }

    pub fn format(&self, vars: &[Var]) -> String {
        assert!(self.var_len() <= vars.len());

        let mut terms = vec![];

        if self.constant() != 0 || self.is_constant() {
            terms.push(format!("{}", self.constant()));
        }

        for (i, coeff) in self.non_constants().iter().enumerate() {
            let var = vars[i];
            if *coeff < 0 {
                if *coeff == -1 {
                    terms.push(format!("- {}", var));
                } else {
                    terms.push(format!("- {}{}", -coeff, var));
                }
            } else if *coeff > 0 {
                if !terms.is_empty() {
                    terms.push("+".into());
                }
                if *coeff == 1 {
                    terms.push(var.to_string());
                } else {
                    terms.push(format!("{}{}", coeff, var));
                }
            }
        }

        terms.join(" ")
    }
}

impl Sub for &Affine {
    type Output = Affine;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.len(), rhs.len());
        let coeffs: Vec<_> = (0..len)
            .map(|i| self.coeffs.get(i).unwrap_or(&0) - rhs.coeffs.get(i).unwrap_or(&0))
            .collect();
        Affine::new(coeffs)
    }
}

impl Neg for &Affine {
    type Output = Affine;

    fn neg(self) -> Self::Output {
        let coeffs = self.coeffs.iter().map(|coeff| -coeff).collect();
        Affine::new(coeffs)
    }
}

impl Mul<Coeff> for &Affine {
    type Output = Affine;

    fn mul(self, rhs: Coeff) -> Self::Output {
        let coeffs = self.coeffs.iter().map(|coeff| coeff * rhs).collect();
        Affine::new(coeffs)
    }
}
