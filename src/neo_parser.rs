use chumsky::prelude::*;
use chumsky::Parser;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Whitespace(String),
}

pub fn empty_line() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    newline().chain(newline())
}

pub fn following_chars() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    // Used to pull characters in a word or string after the
    // starting characters
    none_of(" \n\t")
        .repeated()
        .collect::<String>()
        .map_with_span(|val, span| (Token::Text(val), span))
        .repeated()
        .exactly(1)
}

pub fn initial_chars() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    // The start of a word with either a single character or
    // a single "<" followed by a non "<" character
    non_less_than_char().or(less_than_with_char())
}

pub fn less_than_with_char() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>>
{
    // for finding a less than at the start of a word
    // along with the next letter in the word to allow
    // parsing to continue properly
    just('<')
        .map_with_span(|val, span| (Token::Text(val.to_string()), span))
        .chain(non_less_than_char())
}

pub fn newline() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    just("\n")
        .map_with_span(|val, span| (Token::Whitespace(val.to_string()), span))
        .repeated()
        .exactly(1)
}

pub fn non_less_than_char() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    // word characters that aren't a lessthan or whitespace
    none_of("< \n\t")
        .repeated()
        .exactly(1)
        .collect::<String>()
        .map_with_span(|val, span| (Token::Text(val), span))
        .repeated()
        .exactly(1)
}

pub fn paragraph() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    word().separated_by(wordbreak()).flatten()
}

pub fn word() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    // assembles individual words
    initial_chars().chain(following_chars())
}

pub fn whitespace() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    just(" ")
        .or(just("\t"))
        .map_with_span(|val, span| (Token::Whitespace(val.to_string()), span))
        .repeated()
        .exactly(1)
}

pub fn wordbreak() -> impl Parser<char, Vec<(Token, Range<usize>)>, Error = Simple<char>> {
    whitespace().or(newline())
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

    #[test]
    fn initial_chars_xxx_starting_with_letter() {
        let src = "foxtrot";
        let left = Some(vec![(Token::Text("f".to_string()), 0..1)]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn initial_chars_xxx_test_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            (Token::Text("<".to_string()), 0..1),
            (Token::Text("h".to_string()), 1..2),
        ]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn following_chars_xxx_basic_test() {
        let src = "elta echo";
        let left = Some(vec![(Token::Text("elta".to_string()), 0..4)]);
        let (right, _err) = following_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn word_xxx_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            (Token::Text("<".to_string()), 0..1),
            (Token::Text("h".to_string()), 1..2),
            (Token::Text("otel".to_string()), 2..6),
        ]);
        let (right, _err) = word().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn whitespace_xxx_single_space() {
        let src = " ";
        let left = Some(vec![(Token::Whitespace(" ".to_string()), 0..1)]);
        let (right, _err) = whitespace().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn newline_xxx_basic_() {
        let src = "\n";
        let left = Some(vec![(Token::Whitespace("\n".to_string()), 0..1)]);
        let (right, _err) = newline().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_whitespace() {
        let src = " ";
        let left = Some(vec![(Token::Whitespace(" ".to_string()), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_newline() {
        let src = "\n";
        let left = Some(vec![(Token::Whitespace("\n".to_string()), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn empty_line_xxx_basic_test() {
        let src = "\n\n";
        let left = Some(vec![
            (Token::Whitespace("\n".to_string()), 0..1),
            (Token::Whitespace("\n".to_string()), 1..2),
        ]);
        let (right, _err) = empty_line().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn paragraph_xxx_basic() {
        let src = "charlie papa mike";
        let left = Some(vec![
            (Token::Text("c".to_string()), 0..1),
            (Token::Text("harlie".to_string()), 1..7),
            (Token::Text("p".to_string()), 8..9),
            (Token::Text("apa".to_string()), 9..12),
            (Token::Text("m".to_string()), 13..14),
            (Token::Text("ike".to_string()), 14..17),
        ]);
        let (right, _err) = paragraph().parse_recovery(src);
        assert_eq!(left, right);
    }

    //
}
