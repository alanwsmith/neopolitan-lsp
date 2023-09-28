use chumsky::prelude::*;
use chumsky::Parser;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum NeoToken {
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

pub fn neo_parse(source: &str) -> (Option<Vec<NeoToken>>, Vec<Simple<char>>) {
    title_section().parse_recovery(source)
}

pub fn dashes() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // two dashes followed by a single space as a leader for sections
    // and section attributes
    just("--")
        .map_with_span(|val, span| NeoToken::Decorator(val.to_string(), span))
        .then_ignore(just(" ").repeated().at_least(1))
        .repeated()
        .exactly(1)
}

pub fn empty_line() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    newline().chain(newline())
}

pub fn following_chars() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // Used to pull characters in a word or string after the
    // starting characters
    none_of(" \n\t")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map_with_span(|val, span| NeoToken::String(val, span))
        .repeated()
        .exactly(1)
}

pub fn h1_section() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    h1_section_heading()
        .then_ignore(empty_line())
        .chain(paragraph())
}

pub fn h1_section_heading() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    dashes().chain(just("h1").map_with_span(|val, span| NeoToken::Class(val.to_string(), span)))
}

// // pub fn section_parser() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
// pub fn section_parser() -> impl Recursive<char, _, Vec<NeoToken>, _, Error = Simple<char>> {
//     // recursive(|_s| section2().repeated().at_least(1).flatten())
//     recursive(|_s| just("asdf").map(|_| "asdf".to_string()))
// }

pub fn h2_section() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    h2_section_heading()
        .then_ignore(empty_line())
        .chain(section2())
    // .map(|(a, b)| vec![])
}

pub fn h2_section_heading() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    dashes().chain(just("h2").map_with_span(|val, span| NeoToken::Class(val.to_string(), span)))
}

pub fn initial_chars() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // The start of a word with either a single character or
    // a single "<" followed by a non "<" character
    non_less_than_char().or(less_than_with_char())
}

// pub fn initial_paragraph_word() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
//     just("-- ").not()
//     // non_less_than_char().or(less_than_with_char())
// }

pub fn less_than_with_char() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // for finding a less than at the start of a word
    // along with the next letter in the word to allow
    // parsing to continue properly
    just('<')
        .map_with_span(|val, span| NeoToken::String(val.to_string(), span))
        .chain(non_less_than_char())
}

pub fn newline() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    just("\n")
        .map_with_span(|val, span| NeoToken::Whitespace(val.to_string(), span))
        .repeated()
        .exactly(1)
}

pub fn non_less_than_char() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // word characters that aren't a lessthan or whitespace
    none_of("< \n\t")
        .repeated()
        .exactly(1)
        .collect::<String>()
        .map_with_span(|val, span| NeoToken::String(val, span))
        .repeated()
        .exactly(1)
}

pub fn paragraph() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    word()
        .separated_by(wordbreak())
        .at_least(1)
        .flatten()
        .then_ignore(empty_line().or_not())
}

pub fn section_body_paragraphs() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    paragraph().repeated().flatten()
}

pub fn section() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    title_section()
        .or(h1_section())
        // .or(h2_section())
        .repeated()
        .at_least(1)
        .flatten()
}

pub fn section2() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    let sp = recursive(|sp| {
        title_section()
            .or(h2_section())
            .repeated()
            .at_least(1)
            .flatten()

        // title_section()
        //     .or(h1_section())
        //     // .or(h2_section())
        //     .repeated()
        //     .at_least(1)
        //     .flatten()
    });
    sp
    //sp.parse("asdf")
}

// pub fn sections() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
//     let s = recursive(|x| {
//         section()
//         // .title_section()
//         // .or(h1_section())
//         // .or(h2_section())
//         // .repeated()
//         // .at_least(1)
//         // .flatten()
//     });
//     s
// }

pub fn title_section() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    title_section_heading()
        .then_ignore(empty_line())
        .chain(paragraph())
}

pub fn title_section_heading() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    dashes().chain(just("title").map_with_span(|val, span| NeoToken::Class(val.to_string(), span)))
}

pub fn word() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    // assembles individual words
    // deal with cases with a leading "<" then
    // do just initial_chars which after it
    initial_chars().chain(following_chars()).or(initial_chars())
}

pub fn whitespace() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    just(" ")
        .or(just("\t"))
        .map_with_span(|val, span| NeoToken::Whitespace(val.to_string(), span))
        .repeated()
        .exactly(1)
}

pub fn wordbreak() -> impl Parser<char, Vec<NeoToken>, Error = Simple<char>> {
    whitespace().or(newline())
}

#[cfg(test)]

mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn dashes_xxx_basic() {
        let src = "-- ";
        let left = Some(vec![(NeoToken::Decorator("--".to_string(), 0..2))]);
        let (right, _err) = dashes().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn non_less_than_char_xxx_test_letter() {
        let src = "a";
        let left = Some(vec![(NeoToken::String("a".to_string(), 0..1))]);
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
            NeoToken::String("<".to_string(), 0..1),
            NeoToken::String("b".to_string(), 1..2),
        ]);
        let (right, _err) = less_than_with_char().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn initial_chars_xxx_starting_with_letter() {
        let src = "foxtrot";
        let left = Some(vec![NeoToken::String("f".to_string(), 0..1)]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn initial_chars_xxx_test_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            NeoToken::String("<".to_string(), 0..1),
            NeoToken::String("h".to_string(), 1..2),
        ]);
        let (right, _err) = initial_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn following_chars_xxx_basic_test() {
        let src = "elta echo";
        let left = Some(vec![NeoToken::String("elta".to_string(), 0..4)]);
        let (right, _err) = following_chars().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn paragraph_start_word_xxx_allowed() {
        let src = "foxtrot";
        let left = Some(vec![
            NeoToken::String("f".to_string(), 0..1),
            NeoToken::String("oxtrot".to_string(), 1..7),
        ]);
        let (right, _err) = word().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn word_xxx_with_leading_lt() {
        let src = "<hotel";
        let left = Some(vec![
            NeoToken::String("<".to_string(), 0..1),
            NeoToken::String("h".to_string(), 1..2),
            NeoToken::String("otel".to_string(), 2..6),
        ]);
        let (right, _err) = word().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn word_xxx_single_character() {
        let src = "a";
        let left = Some(vec![NeoToken::String("a".to_string(), 0..1)]);
        let (right, _err) = word().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn whitespace_xxx_single_space() {
        let src = " ";
        let left = Some(vec![NeoToken::Whitespace(" ".to_string(), 0..1)]);
        let (right, _err) = whitespace().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn newline_xxx_basic() {
        let src = "\n";
        let left = Some(vec![NeoToken::Whitespace("\n".to_string(), 0..1)]);
        let (right, _err) = newline().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_whitespace() {
        let src = " ";
        let left = Some(vec![NeoToken::Whitespace(" ".to_string(), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    fn wordbreak_xxx_with_newline() {
        let src = "\n";
        let left = Some(vec![NeoToken::Whitespace("\n".to_string(), 0..1)]);
        let (right, _err) = wordbreak().parse_recovery(src);
        assert_eq!(left, right);
    }

    // #[test]
    // fn initial_paragraph_word_xxx_allowed_word() {
    //     let src = "foxtrot";
    //     let left = Some(vec![
    //         NeoToken::String("f".to_string(), 0..1),
    //         NeoToken::String("oxtrot".to_string(), 0..1),
    //     ]);
    //     let (right, _err) = initial_paragraph_word().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    #[test]
    fn empty_line_xxx_basic_test() {
        let src = "\n\n";
        let left = Some(vec![
            NeoToken::Whitespace("\n".to_string(), 0..1),
            NeoToken::Whitespace("\n".to_string(), 1..2),
        ]);
        let (right, _err) = empty_line().parse_recovery(src);
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn paragraph_xxx_basic() {
        let src = "charlie papa mike";
        let left = Some(vec![
            NeoToken::String("c".to_string(), 0..1),
            NeoToken::String("harlie".to_string(), 1..7),
            NeoToken::String("p".to_string(), 8..9),
            NeoToken::String("apa".to_string(), 9..12),
            NeoToken::String("m".to_string(), 13..14),
            NeoToken::String("ike".to_string(), 14..17),
        ]);
        let (right, _err) = paragraph().parse_recovery(src);
        assert_eq!(left, right);
    }

    // #[test]
    // fn section_name_xxx_basic() {
    //     let src = "title";
    //     let left = Some(vec![NeoToken::Class("title".to_string(), 0..5)]);
    //     let (right, _err) = section_name().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn section_start_xxx_basic() {
    //     let src = "-- code";
    //     let left = Some(vec![
    //         NeoToken::Decorator("--".to_string(), 0..2),
    //         NeoToken::Class("code".to_string(), 3..7),
    //     ]);
    //     let (right, _err) = section_start().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    ///////////////////////////////////////////
    /// Sections
    ///////////////////////////////////////////

    #[test]
    fn section_xxx_two_section() {
        let src = "-- title\n\nalfa bravo\n\n-- h1\n\n charlie delta";
        let left = Some(vec![
            NeoToken::Decorator("--".to_string(), 0..2),
            NeoToken::Class("title".to_string(), 3..8),
            NeoToken::String("a".to_string(), 10..11),
            NeoToken::String("lfa".to_string(), 11..14),
            NeoToken::String("b".to_string(), 15..16),
            NeoToken::String("ravo".to_string(), 16..20),
        ]);
        // let section_parser = recursive(|_s| section2());
        // dbg!(section_parser.parse(src));

        // let (right, _err) = section_parser().parse(src);
        // assert_eq!(left, right);
    }

    #[test]
    fn h1_section_heading_xxx_basic() {
        let src = "-- h1";
        let left = Some(vec![
            NeoToken::Decorator("--".to_string(), 0..2),
            NeoToken::Class("h1".to_string(), 3..5),
        ]);
        let (right, _err) = h1_section_heading().parse_recovery(src);
        assert_eq!(left, right);
    }

    // #[test]
    // fn h1_section_xxx_basic() {
    //     let src = "-- h1\n\nalfa bravo charlie";
    //     let left = Some(vec![
    //         NeoToken::Decorator("--".to_string(), 0..2),
    //         NeoToken::Class("title".to_string(), 3..8),
    //         NeoToken::String("a".to_string(), 10..11),
    //         NeoToken::String("lfa".to_string(), 11..14),
    //         NeoToken::String("b".to_string(), 15..16),
    //         NeoToken::String("ravo".to_string(), 16..20),
    //         NeoToken::String("c".to_string(), 21..22),
    //         NeoToken::String("harlie".to_string(), 22..28),
    //     ]);
    //     let (right, _err) = h1_section().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    #[test]
    fn title_section_heading_xxx_basic() {
        let src = "-- title";
        let left = Some(vec![
            NeoToken::Decorator("--".to_string(), 0..2),
            NeoToken::Class("title".to_string(), 3..8),
        ]);
        let (right, _err) = title_section_heading().parse_recovery(src);
        assert_eq!(left, right);
    }

    // #[test]
    // fn section_body_paragraphs_xxx_basic() {
    //     let src = "a b c\n\nd e";
    //     let left = Some(vec![
    //         NeoToken::String("a".to_string(), 0..1),
    //         NeoToken::String("b".to_string(), 2..3),
    //         NeoToken::String("c".to_string(), 4..5),
    //         NeoToken::String("d".to_string(), 7..8),
    //         NeoToken::String("e".to_string(), 9..10),
    //     ]);
    //     let (right, _err) = section_body_paragraphs().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // // #[ignore]
    // fn section_body_paragraphs_xxx_stop_at_next_section() {
    //     let src = "a b c\n\n-- title";
    //     let left = Some(vec![
    //         NeoToken::String("a".to_string(), 0..1),
    //         NeoToken::String("b".to_string(), 2..3),
    //         NeoToken::String("c".to_string(), 4..5),
    //     ]);
    //     let (right, _err) = section_body_paragraphs().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn section_body_paragraphs_xxx_basic() {
    //     let src = "a b c\n\nd e";
    //     let left = Some(vec![
    //         NeoToken::String("a".to_string(), 0..1),
    //         NeoToken::String("b".to_string(), 2..3),
    //         NeoToken::String("c".to_string(), 4..5),
    //         NeoToken::String("d".to_string(), 21..22),
    //         NeoToken::String("e".to_string(), 21..22),
    //     ]);
    //     let (right, _err) = section_body_paragraphs().parse_recovery(src);
    //     assert_eq!(left, right);
    // }

    //
}
