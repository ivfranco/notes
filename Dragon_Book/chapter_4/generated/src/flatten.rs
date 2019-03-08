use std::error::Error;

lalrpop_mod!(pub lists);

pub fn flatten<'a>(s: &'a str) -> Result<Vec<char>, Box<Error + 'a>> {
    lists::SParser::new().parse(s).map_err(|e| e.into())
}

#[test]
fn flat_list_test() {
    assert_eq!(
        flatten("(a, b, c, (d, e, f))").ok(),
        Some(vec!['a', 'b', 'c', 'd', 'e', 'f'])
    );

    assert!(flatten("a, b, c").is_err());
}
