use super::Parser;

pub fn many1<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
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
pub fn many0<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
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
#[cfg(test)]
mod tests {
    use crate::helper::parser::character::{tag, unsigned_integer};

    use super::*;
    #[test]
    fn many1_test() {
        let parser = many1(tag("ha"));
        assert_eq!(Ok(("", vec!["ha", "ha", "ha"])), parser.parse("hahaha"));
        assert_eq!(Err("ahah"), parser.parse("ahah"));
        assert_eq!(Err(""), parser.parse(""));
    }

    #[test]
    fn many0_test() {
        let parser = many0(tag("ha"));
        assert_eq!(Ok(("", vec!["ha", "ha", "ha"])), parser.parse("hahaha"));
        assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
    }
    #[test]
    fn separated_list0_combinator() {
        let parser = separated_list0(tag(","), unsigned_integer::<u32>);
        assert_eq!(Ok(("", vec![1, 2, 3, 4])), parser.parse("1,2,3,4"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
        assert_eq!(Ok(("", vec![1])), parser.parse("1"));
        assert_eq!(Ok((",", vec![1])), parser.parse("1,"));
        assert_eq!(Ok((",", vec![])), parser.parse(","));
        assert_eq!(Ok((",1", vec![])), parser.parse(",1"));
    }
    #[test]
    fn separated_list1_combinator() {
        let parser = separated_list1(tag(","), unsigned_integer::<u32>);
        assert_eq!(Ok(("", vec![1, 2, 3, 4])), parser.parse("1,2,3,4"));
        assert_eq!(Err(""), parser.parse(""));
        assert_eq!(Ok(("", vec![1])), parser.parse("1"));
        assert_eq!(Ok((",", vec![1])), parser.parse("1,"));
        assert_eq!(Err(","), parser.parse(","));
        assert_eq!(Err(",1"), parser.parse(",1"));
    }
}
