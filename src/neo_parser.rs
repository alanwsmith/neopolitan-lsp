use chumsky::prelude::*;
use chumsky::Parser;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
}

pub fn non_less_than_char() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    none_of("< \n\t")
        .repeated()
        .exactly(1)
        .collect::<String>()
        .map_with_span(|val, span| (Token::Text(val), span))
        .repeated()
        .exactly(1)
}

pub fn less_than_with_char() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>>
{
    just('<')
        .map_with_span(|val, span| (Token::Text(val.to_string()), span))
        .chain(non_less_than_char())
}

#[cfg(test)]

mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn basic() {
        let left = "s";
        let right = "s";
        assert_eq!(left, right);
    }

    #[test]
    fn non_less_than_char_xxx_test_letter() {
        let src = "a";
        let left = Some(vec![(Token::Text("a".to_string()), 0..1)]);
        let (right, _err) = non_less_than_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn non_less_than_char_xxx_test_skip_lt_char() {
        let src = "<";
        let left = None;
        let (right, _err) = non_less_than_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn less_than_with_char_xxx_basic_test() {
        let src = "<b";
        let left = Some(vec![
            (Token::Text("<".to_string()), 0..1),
            (Token::Text("b".to_string()), 1..2),
        ]);
        let (right, _err) = less_than_with_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    //
}
