use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    combinator::all_consuming,
    error::VerboseError,
    multi::fold_many0,
    sequence::tuple,
    IResult,
};

use crate::utils::{natural, Num};

fn factor(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    fn paren_delimited(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
        let (input, (_, _, e, _, _)) =
            tuple((char('('), multispace0, expr, multispace0, char(')')))(input.trim())?;
        Ok((input, e))
    }
    alt((natural, paren_delimited))(input.trim())
}

#[test]
fn test_factor() {
    assert_eq!(factor("52"), Ok(("", 52)));
    assert_eq!(factor("123 + 9"), Ok((" + 9", 123)));
    assert_eq!(factor("123 * 9"), Ok((" * 9", 123)));
    assert_eq!(factor("(1+2) * 8"), Ok((" * 8", 3)));
    assert_eq!(factor("(1*(2)) * 8"), Ok((" * 8", 2)));
    assert_eq!(
        factor("(1*(2 + 2)) * (100 + 200)"),
        Ok((" * (100 + 200)", 4))
    );
    assert!(factor("+ 4").is_err());
}

fn term(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    let (input, head) = factor(input.trim())?;

    fold_many0(
        tuple((
            multispace0,
            alt((tag("*"), tag("/"))),
            multispace0,
            factor,
            multispace0,
        )),
        move || head,
        |acc, item| {
            let (_, op, _, v, _) = item;
            match op {
                "*" => acc * v,
                "/" => acc / v,
                _ => unreachable!("unknown operator"),
            }
        },
    )(input.trim())
}

#[test]
fn test_term() {
    assert_eq!(term("43"), Ok(("", 43)));
    assert_eq!(term("4 * 3"), Ok(("", 12)));
    assert_eq!(term("(1 + 2) * (3 * 4)"), Ok(("", 36)));
    assert_eq!(term("2 * 3 * 3"), Ok(("", 18)));
    assert_eq!(term("4 / 3"), Ok(("", 1)));
    assert_eq!(term("(3 * 4) / (1+2)"), Ok(("", 4)));
    assert_eq!(term("2 / 3 / 3"), Ok(("", 0)));
}

fn expr(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    let (input, head) = term(input.trim())?;

    fold_many0(
        tuple((
            multispace0,
            alt((tag("+"), tag("-"))),
            multispace0,
            term,
            multispace0,
        )),
        move || head,
        |acc, item| {
            let (_, op, _, v, _) = item;
            match op {
                "+" => acc + v,
                "-" => acc - v,
                _ => unreachable!("unknown operator"),
            }
        },
    )(input.trim())
}

#[test]
fn test_expr() {
    assert_eq!(expr("100"), Ok(("", 100)));
    assert_eq!(expr("2 + 4"), Ok(("", 6)));
    assert_eq!(expr("10 + (4+4)"), Ok(("", 18)));
    assert_eq!(expr("10 + 4+4"), Ok(("", 18)));
    assert_eq!(expr("2 - 4"), Ok(("", -2)));
    assert_eq!(expr("10 - (4/4)"), Ok(("", 9)));
}

pub fn parse(input: &str) -> IResult<&str, Num, VerboseError<&str>> {
    all_consuming(expr)(input.trim())
}
