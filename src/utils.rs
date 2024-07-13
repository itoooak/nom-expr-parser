use nom::{character::complete::digit1, combinator::map_res, IResult};

pub type Num = i32;

pub fn natural(input: &str) -> IResult<&str, Num> {
    map_res(digit1, str::parse)(input.trim())
}

#[test]
fn test_natural() {
    assert_eq!(natural("52"), Ok(("", 52)));
    assert_eq!(natural("  123 +9 "), Ok((" +9", 123)));
    assert_eq!(natural("  123*9 "), Ok(("*9", 123)));
    assert!(natural("* 4").is_err());
}
