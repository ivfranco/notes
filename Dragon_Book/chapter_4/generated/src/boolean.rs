use std::error::Error;

lalrpop_mod!(pub bexpr);

pub fn eval_boolean_expression<'a>(s: &'a str) -> Result<bool, Box<Error + 'a>> {
    bexpr::BExprParser::new().parse(s).map_err(|e| e.into())
}

#[test]
fn boolean_expr_evaluation_test() {
    assert_eq!(
        eval_boolean_expression("false or true and false").ok(),
        Some(false)
    );
    assert_eq!(
        eval_boolean_expression("false or (not false and true)").ok(),
        Some(true),
    );
    assert!(eval_boolean_expression("not not and false").is_err());
}
