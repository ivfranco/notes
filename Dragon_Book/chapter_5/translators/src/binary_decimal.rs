use std::error::Error;

lalrpop_mod!(pub decimal);

pub fn eval_decimal<'a>(s: &'a str) -> Result<f64, Box<Error + 'a>> {
    decimal::SParser::new().parse(s).map_err(|e| e.into())
}

#[test]
fn eval_decimal_test() {
    const EPSILON: f64 = 1e-5;
    assert!((eval_decimal("101.101").unwrap() - 5.625).abs() < EPSILON);
}
