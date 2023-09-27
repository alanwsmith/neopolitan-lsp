use chumsky::prelude::*;
use chumsky::Parser;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Token {
    Class(String, Range<usize>),
    Comment(String, Range<usize>),
    Decorator(String, Range<usize>),
    Enum(String, Range<usize>),
    EnumMember(String, Range<usize>),
    Event(String, Range<usize>),
    Function(String, Range<usize>),
    Interface(String, Range<usize>),
    Keyword(String, Range<usize>),
    Macro(String, Range<usize>),
    Method(String, Range<usize>),
    Modifier(String, Range<usize>),
    Namespace(String, Range<usize>),
    Number(String, Range<usize>),
    Operator(String, Range<usize>),
    Parameter(String, Range<usize>),
    Property(String, Range<usize>),
    Regexp(String, Range<usize>),
    String(String, Range<usize>),
    Struct(String, Range<usize>),
    Type(String, Range<usize>),
    TypeParameter(String, Range<usize>),
    Variable(String, Range<usize>),

    // whitespace is not part of the spec
    // it's here for the parsing only
    Whitespace(String, Range<usize>),
}

pub fn dashes() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // two dashes followed by a single space as a leader for sections
    // and section attributes
    just("--")
        .map_with_span(|val, span| Token::Decorator(val.to_string(), span))
        .then_ignore(just(" ").repeated().at_least(1))
        .repeated()
        .exactly(1)
}

pub fn empty_line() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    newline().chain(newline())
}

pub fn following_chars() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // Used to pull characters in a word or string after the
    // starting characters
    none_of(" \n\t")
        .repeated()
        .collect::<String>()
        .map_with_span(|val, span| Token::String(val, span))
        .repeated()
        .exactly(1)
}

pub fn initial_chars() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // The start of a word with either a single character or
    // a single "<" followed by a non "<" character
    non_less_than_char().or(less_than_with_char())
}

pub fn less_than_with_char() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // for finding a less than at the start of a word
    // along with the next letter in the word to allow
    // parsing to continue properly
    just('<')
        .map_with_span(|val, span| Token::String(val.to_string(), span))
        .chain(non_less_than_char())
}

pub fn newline() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    just("\n")
        .map_with_span(|val, span| Token::Whitespace(val.to_string(), span))
        .repeated()
        .exactly(1)
}

pub fn non_less_than_char() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // word characters that aren't a lessthan or whitespace
    none_of("< \n\t")
        .repeated()
        .exactly(1)
        .collect::<String>()
        .map_with_span(|val, span| Token::String(val, span))
        .repeated()
        .exactly(1)
}

pub fn paragraph() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    word().separated_by(wordbreak()).flatten()
}

pub fn section_name() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    just("title")
        .or(just("code"))
        .or(just("h2"))
        .map_with_span(|val, span| Token::Class(val.to_string(), span))
        .repeated()
        .exactly(1)
}

pub fn section_start() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    dashes().chain(section_name())
}

pub fn title_section() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    title_section_heading()
        .then_ignore(empty_line())
        .chain(paragraph())
}

pub fn title_section_heading() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    dashes().chain(just("title").map_with_span(|val, span| Token::Class(val.to_string(), span)))
}

pub fn word() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    // assembles individual words
    initial_chars().chain(following_chars())
}

pub fn whitespace() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    just(" ")
        .or(just("\t"))
        .map_with_span(|val, span| Token::Whitespace(val.to_string(), span))
        .repeated()
        .exactly(1)
}

pub fn wordbreak() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    whitespace().or(newline())
}

#[cfg(test)]

mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn dashes_xxx_basic() {
        let src = "-- ";
        let left = Some(vec![(Token::Decorator("--".to_string(), 0..2))]);
        let (right, _err) = dashes().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn non_less_than_char_xxx_test_letter() {
        let src = "a";
        let left = Some(vec![(Token::String("a".to_string(), 0..1))]);
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
            Token::String("<".to_string(), 0..1),
            Token::String("b".to_string(), 1..2),
        ]);
        let (right, _err) = less_than_with_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn initial_chars_xxx_starting_with_letter() {
        let src = "foxtrot";
        let left = Some(vec![Token::String("f".to_string(), 0..1)]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn initial_chars_xxx_test_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            Token::String("<".to_string(), 0..1),
            Token::String("h".to_string(), 1..2),
        ]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn following_chars_xxx_basic_test() {
        let src = "elta echo";
        let left = Some(vec![Token::String("elta".to_string(), 0..4)]);
        let (right, _err) = following_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn word_xxx_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            Token::String("<".to_string(), 0..1),
            Token::String("h".to_string(), 1..2),
            Token::String("otel".to_string(), 2..6),
        ]);
        let (right, _err) = word().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn whitespace_xxx_single_space() {
        let src = " ";
        let left = Some(vec![Token::Whitespace(" ".to_string(), 0..1)]);
        let (right, _err) = whitespace().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn newline_xxx_basic() {
        let src = "\n";
        let left = Some(vec![Token::Whitespace("\n".to_string(), 0..1)]);
        let (right, _err) = newline().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_whitespace() {
        let src = " ";
        let left = Some(vec![Token::Whitespace(" ".to_string(), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_newline() {
        let src = "\n";
        let left = Some(vec![Token::Whitespace("\n".to_string(), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn empty_line_xxx_basic_test() {
        let src = "\n\n";
        let left = Some(vec![
            Token::Whitespace("\n".to_string(), 0..1),
            Token::Whitespace("\n".to_string(), 1..2),
        ]);
        let (right, _err) = empty_line().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn paragraph_xxx_basic() {
        let src = "charlie papa mike";
        let left = Some(vec![
            Token::String("c".to_string(), 0..1),
            Token::String("harlie".to_string(), 1..7),
            Token::String("p".to_string(), 8..9),
            Token::String("apa".to_string(), 9..12),
            Token::String("m".to_string(), 13..14),
            Token::String("ike".to_string(), 14..17),
        ]);
        let (right, _err) = paragraph().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn section_name_xxx_basic() {
        let src = "title";
        let left = Some(vec![Token::Class("title".to_string(), 0..5)]);
        let (right, _err) = section_name().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn section_start_xxx_basic() {
        let src = "-- code";
        let left = Some(vec![
            Token::Decorator("--".to_string(), 0..2),
            Token::Class("code".to_string(), 3..7),
        ]);
        let (right, _err) = section_start().parse_recovery(src);
        assert_eq!(left, right);
    }

    ///////////////////////////////////////////
    /// Secitons
    ///////////////////////////////////////////

    #[test]
    fn title_section_heading_xxx_basic() {
        let src = "-- title";
        let left = Some(vec![
            Token::Decorator("--".to_string(), 0..2),
            Token::Class("title".to_string(), 3..8),
        ]);
        let (right, _err) = title_section_heading().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn title_section_xxx_basic() {
        let src = "-- title\n\nalfa bravo charlie";
        let left = Some(vec![
            Token::Decorator("--".to_string(), 0..2),
            Token::Class("title".to_string(), 3..8),
            Token::String("a".to_string(), 10..11),
            Token::String("lfa".to_string(), 11..14),
            Token::String("b".to_string(), 15..16),
            Token::String("ravo".to_string(), 16..20),
            Token::String("c".to_string(), 21..22),
            Token::String("harlie".to_string(), 22..28),
        ]);
        let (right, _err) = title_section().parse_recovery(src);
        assert_eq!(left, right);
    }

    //
}
