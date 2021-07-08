use std::collections::HashMap;

use nom::{bytes::complete::take_while, character::complete::space0, IResult, Parser};

const REQUEST_COMMON_TAIL: &str = " con = XXX len = XXX";
const REPLY_COMMON_TAIL: &str = " status=XXX pl = XXX con = XXX len = XXX";

#[derive(Debug)]
enum Ty {
    Request,
    Reply(bool),
}

#[derive(Debug)]
pub struct Trace {
    epoch: f64,
    from: String,
    to: String,
    ty: Ty,
    session_id: u32,
    op_code: u32,
    operation: String,
    params: HashMap<String, String>,
}

impl Trace {
    pub fn parse(input: &str) -> Result<Trace, String> {
        p_trace(input)
            .map(|(_, trace)| trace)
            .map_err(|e| e.to_string())
    }
}

fn lexeme(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    take_while(|c: char| !c.is_whitespace())(input)
}

fn p_trace(input: &str) -> IResult<&str, Trace> {
    let (input, epoch) = lexeme.map(|s| s.parse::<f64>().unwrap()).parse(input)?;
    let (input, from) = lexeme.map(|s| s.to_string()).parse(input)?;
    let (input, to) = lexeme.map(|s| s.to_string()).parse(input)?;
    // skip "U"
    let (input, _) = lexeme(input)?;
    let (input, mut ty) = lexeme
        .map(|s| match s {
            "C3" => Ty::Request,
            "R3" => Ty::Reply(true),
            _ => unreachable!(),
        })
        .parse(input)?;
    let (input, session_id) = lexeme
        .map(|s| u32::from_str_radix(s, 16).unwrap())
        .parse(input)?;
    let (input, op_code) = lexeme.map(|s| s.parse::<u32>().unwrap()).parse(input)?;
    let (input, operation) = lexeme.map(|s| s.to_string()).parse(input)?;

    let input = match ty {
        Ty::Request => input.strip_suffix(REQUEST_COMMON_TAIL).unwrap(),
        Ty::Reply(..) => input.strip_suffix(REPLY_COMMON_TAIL).unwrap(),
    };

    let mut input = match ty {
        Ty::Request => {
            let (input, _) = space0(input)?;
            input
        }
        Ty::Reply(_) => {
            let (input, ok) = lexeme.map(|s| s == "OK").parse(input)?;
            ty = Ty::Reply(ok);
            input
        }
    };

    let mut params = HashMap::new();

    while !input.is_empty() {
        let (remain, key) = lexeme.map(|s| s.to_string()).parse(input)?;
        let (remain, value) = lexeme.map(|s| s.to_string()).parse(remain)?;
        input = remain;
        params.insert(key, value);
    }

    let trace = Trace {
        epoch,
        from,
        to,
        ty,
        session_id,
        op_code,
        operation,
        params,
    };

    Ok((input, trace))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "1034787600.774001 30.0801 31.0320 U R3 d2891970 1 getattr OK ftype 1 mode 180 nlink 1 uid 18a88 gid 18a88 size e7e used 1000 rdev 0 rdev2 0 fsid 8664 fileid 355861 atime 1034786445.038001 mtime 1034786444.934008 ctime 1034786444.934008 status=XXX pl = XXX con = XXX len = XXX";
        println!("{:?}", Trace::parse(input).unwrap());
    }
}
