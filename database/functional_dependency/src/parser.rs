use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    error::ParseError,
    multi::separated_list1,
    AsChar, IResult, InputTakeAtPosition, Parser,
};

fn ident(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(input)
}

fn lexeme<P, I, O, E>(mut parser: P) -> impl FnMut(I) -> IResult<I, O, E>
where
    P: Parser<I, O, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    move |input: I| {
        let (input, _) = space0(input)?;
        parser.parse(input)
    }
}

fn sep_by_comma(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(lexeme(tag(",")), lexeme(ident))(input)
}

fn dependency<'a>(arrow: &str, input: &'a str) -> IResult<&'a str, (Vec<&'a str>, Vec<&'a str>)> {
    let (input, source) = sep_by_comma(input)?;
    let (input, _) = lexeme(tag(arrow))(input)?;
    let (input, target) = sep_by_comma(input)?;

    Ok((input, (source, target)))
}

// FD <- IDENT ("," IDENT)* "->" IDENT ("," IDENT)*
pub fn fd(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    dependency("->", input)
}

// MVD <- IDENT ("," IDENT)* "->> IDENT ("," IDENT)*
pub fn mvd(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    dependency("->>", input)
}
