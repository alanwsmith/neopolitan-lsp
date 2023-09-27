use chumsky::prelude::*;
use chumsky::Parser;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
}

fn non_less_than_char() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    none_of("< \n\t")
        .repeated()
        .exactly(1)
        .collect::<String>()
        .map_with_span(|val, span| (Token::Text(val), span))
        .repeated()
        .at_least(1)
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
    fn non_less_than_char_test_letter() {
        let src = "a";
        let left = Some(vec![(Token::Text("a".to_string()), 0..1)]);
        let (right, _err) = non_less_than_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    //
}
