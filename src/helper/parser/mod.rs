use combinator::map;
pub mod branch;
pub mod character;
pub mod combinator;
pub mod multi;
pub mod sequence;
pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;
pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
    fn map<F, NewOutput>(self, map_fn: F) -> Boxed<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        Boxed::new(map(self, map_fn))
    }
    fn pred<F>(self, pred_fn: F) -> Boxed<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        Boxed::new(pred(self, pred_fn))
    }
    fn and_then<F, NextParser, NewOutput>(self, f: F) -> Boxed<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        Boxed::new(and_then(self, f))
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
pub struct Boxed<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> Boxed<'a, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        Boxed {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for Boxed<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

/// matches a parser under a certain condition (F).
pub fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input)
            && predicate(&value)
        {
            return Ok((next_input, value));
        }
        Err(input)
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

#[cfg(test)]
mod tests {
    use character::char;

    use super::*;

    #[test]
    fn predicate_combinator() {
        let parser = pred(char, |c| *c == 'o');
        assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
        assert_eq!(Err("lol"), parser.parse("lol"));
    }
}
