#[macro_use]
extern crate lalrpop_util;

use boolean_expression::Expr;
use std::error::Error;

lalrpop_mod!(pub expr);

pub fn parse_expr<'a>(s: &'a str) -> Result<Expr<char>, Box<Error + 'a>> {
    expr::BOrParser::new().parse(s).map_err(Box::from)
}
