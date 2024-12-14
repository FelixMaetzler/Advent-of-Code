use super::{
    character::{multispace0, tag},
    combinator::map,
    Parser,
};

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}
pub fn triple<'a, P1, P2, P3, R1, R2, R3>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Parser<'a, (R1, R2, R3)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
    P3: Parser<'a, R3>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input).and_then(|(next_input, result2)| {
                parser3
                    .parse(next_input)
                    .map(|(last_input, result3)| (last_input, (result1, result2, result3)))
            })
        })
    }
}
/// Matches two Parsers and only returns the result of the left one
pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}
/// Matches two Parsers and only returns the result of the right one
pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}
/// matches <left> <middle> <right> and discards left and right
pub fn delimited<'a, P1, P2, P3, A, R1, R2>(le: P1, middle: P2, ri: P3) -> impl Parser<'a, A>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, A>,
    P3: Parser<'a, R2>,
{
    right(le, left(middle, ri))
}

pub fn separated_pair<'a, P1, P2, R1, R2>(
    parser1: P1,
    sep: &'static str,
    parser2: P2,
) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    let new = left(parser1, tag(sep));
    move |input| {
        new.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}
/// matches <space0> parser <space0>
pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    delimited(multispace0(), parser, multispace0())
}
#[cfg(test)]
mod tests {
    use crate::helper::parser::character::unsigned_integer;

    use super::*;
    #[test]
    fn pair_combinator() {
        let tag_opener = pair(tag("<"), unsigned_integer);
        assert_eq!(Ok(("/>", ("<", 5))), tag_opener.parse("<5/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }
    #[test]
    fn right_combinator() {
        let tag_opener = right(tag("<"), unsigned_integer);
        assert_eq!(Ok(("/>", 5)), tag_opener.parse("<5/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }
}
