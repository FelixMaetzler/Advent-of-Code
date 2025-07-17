use std::str::FromStr;

use super::{
    ParseResult, Parser,
    multi::{many0, many1},
    pred,
};
pub fn char(input: &str) -> ParseResult<char> {
    input
        .chars()
        .next()
        .map_or(Err(input), |next| Ok((&input[next.len_utf8()..], next)))
}
fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(char, |c| c.is_whitespace())
}
pub fn multispace1<'a>() -> impl Parser<'a, Vec<char>> {
    many1(whitespace_char())
}

pub fn multispace0<'a>() -> impl Parser<'a, Vec<char>> {
    many0(whitespace_char())
}
/// succeeds if the start of the input matches the expected one
pub fn tag<'a>(expected: &'static str) -> impl Parser<'a, &'a str> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], next)),
        _ => Err(input),
    }
}
pub fn unsigned_integer<T>(input: &str) -> ParseResult<T>
where
    T: FromStr,
{
    let mut counter = 0;
    for c in input.chars() {
        if c.is_ascii_digit() {
            counter += 1;
        } else {
            break;
        }
    }

    input[..counter]
        .parse()
        .map_or_else(|_| Err(input), |x| Ok((&input[counter..], x)))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tag_test() {
        let parse_felix = tag("Hello Felix!");
        assert_eq!(Ok(("", "Hello Felix!")), parse_felix.parse("Hello Felix!"));
        assert_eq!(
            Ok((" Hello Robert!", "Hello Felix!")),
            parse_felix.parse("Hello Felix! Hello Robert!")
        );
        assert_eq!(Err("Hello Mike!"), parse_felix.parse("Hello Mike!"));
    }
}
