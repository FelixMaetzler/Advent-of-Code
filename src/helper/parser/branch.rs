use super::Parser;

pub fn alt<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}
pub fn alt_vec<'a, P, A>(parsers: &[P]) -> impl Parser<'a, A> + use<'a, '_, P, A>
where
    P: Parser<'a, A>,
{
    move |input| {
        for parser in parsers {
            if let Ok(x) = parser.parse(input) {
                return Ok(x);
            }
        }
        Err(input)
    }
}
