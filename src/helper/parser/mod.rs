use std::str::FromStr;

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;
pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }
    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }
    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}
impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}
pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}
/// succeeds if the start of the input matches the expected one
pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, &'a str> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], next)),
        _ => Err(input),
    }
}

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
pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
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
pub fn middle<'a, P1, P2, P3, A, R1, R2>(le: P1, middle: P2, ri: P3) -> impl Parser<'a, A>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, A>,
    P3: Parser<'a, R2>,
{
    right(le, left(middle, ri))
}
pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}
pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}
pub fn separated_list0<'a, P1, P2, A, B>(sep: P1, parser: P2) -> impl Parser<'a, Vec<B>>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, value)) = parser.parse(input) {
            result.push(value);
            input = next_input;
        } else {
            return Ok((input, result));
        }
        while let Ok((next_input, _)) = sep.parse(input) {
            if let Ok((next_input, value)) = parser.parse(next_input) {
                result.push(value);
                input = next_input;
            } else {
                break;
            }
        }

        Ok((input, result))
    }
}
pub fn separated_list1<'a, P1, P2, A, B>(sep: P1, parser: P2) -> impl Parser<'a, Vec<B>>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, value)) = parser.parse(input) {
            result.push(value);
            input = next_input;
        } else {
            return Err(input);
        }
        while let Ok((next_input, _)) = sep.parse(input) {
            if let Ok((next_input, value)) = parser.parse(next_input) {
                result.push(value);
                input = next_input;
            } else {
                break;
            }
        }

        Ok((input, result))
    }
}
pub fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}
/// matches a parser under a certain condition (F)
pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}
pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}
pub fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}
pub fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}
pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}
/// matches <space0> parser <space0>
pub fn whitespace_wrap<'a, P, A>(parser: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
{
    middle(space0(), parser, space0())
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
    let new = left(parser1, match_literal(sep));
    move |input| {
        new.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
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

    match input[..counter].parse::<T>() {
        Ok(x) => Ok((&input[counter..], x)),
        Err(_) => Err(input),
    }
}
pub fn digit<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_ascii_digit())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn literal_parser() {
        let parse_felix = match_literal("Hello Felix!");
        assert_eq!(Ok(("", "Hello Felix!")), parse_felix.parse("Hello Felix!"));
        assert_eq!(
            Ok((" Hello Robert!", "Hello Felix!")),
            parse_felix.parse("Hello Felix! Hello Robert!")
        );
        assert_eq!(Err("Hello Mike!"), parse_felix.parse("Hello Mike!"));
    }

    #[test]
    fn pair_combinator() {
        let tag_opener = pair(match_literal("<"), unsigned_integer);
        assert_eq!(Ok(("/>", ("<", 5))), tag_opener.parse("<5/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }
    #[test]
    fn right_combinator() {
        let tag_opener = right(match_literal("<"), unsigned_integer);
        assert_eq!(Ok(("/>", 5)), tag_opener.parse("<5/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }
    #[test]
    fn one_or_more_combinator() {
        let parser = one_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec!["ha", "ha", "ha"])), parser.parse("hahaha"));
        assert_eq!(Err("ahah"), parser.parse("ahah"));
        assert_eq!(Err(""), parser.parse(""));
    }

    #[test]
    fn zero_or_more_combinator() {
        let parser = zero_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec!["ha", "ha", "ha"])), parser.parse("hahaha"));
        assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
    }
    #[test]
    fn predicate_combinator() {
        let parser = pred(any_char, |c| *c == 'o');
        assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
        assert_eq!(Err("lol"), parser.parse("lol"));
    }
    #[test]
    fn separated_list0_combinator() {
        let parser = separated_list0(match_literal(","), unsigned_integer::<u32>);
        assert_eq!(Ok(("", vec![1, 2, 3, 4])), parser.parse("1,2,3,4"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
        assert_eq!(Ok(("", vec![1])), parser.parse("1"));
        assert_eq!(Ok((",", vec![1])), parser.parse("1,"));
        assert_eq!(Ok((",", vec![])), parser.parse(","));
        assert_eq!(Ok((",1", vec![])), parser.parse(",1"));
    }
    #[test]
    fn separated_list1_combinator() {
        let parser = separated_list1(match_literal(","), unsigned_integer::<u32>);
        assert_eq!(Ok(("", vec![1, 2, 3, 4])), parser.parse("1,2,3,4"));
        assert_eq!(Err(""), parser.parse(""));
        assert_eq!(Ok(("", vec![1])), parser.parse("1"));
        assert_eq!(Ok((",", vec![1])), parser.parse("1,"));
        assert_eq!(Err(","), parser.parse(","));
        assert_eq!(Err(",1"), parser.parse(",1"));
    }
}
