#![allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub enum NomToken {
    Placeholder,
    Word(String, usize, usize),
}

pub fn following_word_chars(source: Span) -> IResult<Span, Vec<NomToken>> {
    // Any character (including "<") that's not
    // a whitespace or a break
    let (source, start) = position(source)?;
    let (source, val) = is_not(" \n\t\r")(source)?;
    let (source, end) = position(source)?;
    Ok((
        source,
        vec![NomToken::Word(
            val.to_string(),
            start.location_offset(),
            end.location_offset(),
        )],
    ))
}

pub fn initial_word_chars(source: Span) -> IResult<Span, Vec<NomToken>> {
    // get the first character of a word that
    // allows for a "<", but not two in a row
    let (source, response) = alt((non_lt_char, lt_with_non_lt_char))(source)?;
    Ok((source, response))
}

pub fn lt_with_non_lt_char(source: Span) -> IResult<Span, Vec<NomToken>> {
    // A less than with a trailing non less than
    // character
    let (source, start1) = position(source)?;
    let (source, val1) = tag("<")(source)?;
    let (source, end1) = position(source)?;
    let mut response = vec![NomToken::Word(
        val1.to_string(),
        start1.location_offset(),
        end1.location_offset(),
    )];
    let (source, mut second_char) = non_lt_char(source)?;
    response.append(&mut second_char);
    Ok((source, response))
}

pub fn non_lt_char(source: Span) -> IResult<Span, Vec<NomToken>> {
    // Any character (including "<") that's not
    // a whitespace or a break
    let (source, start) = position(source)?;
    let (source, val) = none_of("< \n\t\r")(source)?;
    let (source, end) = position(source)?;
    Ok((
        source,
        vec![NomToken::Word(
            val.to_string(),
            start.location_offset(),
            end.location_offset(),
        )],
    ))
}

#[cfg(test)]

mod test {
    use super::*;
    use pretty_assertions;

    #[test]
    pub fn following_word_chars_test() {
        let source = Span::new("lfa");
        let left = vec![NomToken::Word("lfa".to_string(), 0, 3)];
        let right = following_word_chars(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_initial_word_chars_via_lt() {
        let source = Span::new("<f");
        let left = vec![
            NomToken::Word("<".to_string(), 0, 1),
            NomToken::Word("f".to_string(), 1, 2),
        ];
        let right = initial_word_chars(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn non_less_than_char_test() {
        let source = Span::new("a");
        let left = vec![NomToken::Word("a".to_string(), 0, 1)];
        let right = non_lt_char(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn non_less_than_char_test_skip_lt() {
        let source = Span::new("<");
        match non_lt_char(source) {
            Ok(_) => {
                assert_eq!(1, 2)
            } // this should not have passed
            Err(_) => {
                assert_eq!(1, 1)
            } // got the proper error
        }
    }

    #[test]
    pub fn lt_with_non_lt_chars_test() {
        let source = Span::new("<a");
        let left = vec![
            NomToken::Word("<".to_string(), 0, 1),
            NomToken::Word("a".to_string(), 1, 2),
        ];
        let right = lt_with_non_lt_char(source).unwrap().1;
        assert_eq!(left, right);
    }
}
