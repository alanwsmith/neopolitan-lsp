#![allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub enum NomToken {
    Class(String, usize, usize),
    Comment(String, usize, usize),
    Decorator(String, usize, usize),
    Enum(String, usize, usize),
    EnumMember(String, usize, usize),
    Event(String, usize, usize),
    Function(String, usize, usize),
    Interface(String, usize, usize),
    Keyword(String, usize, usize),
    Macro(String, usize, usize),
    Method(String, usize, usize),
    Modifier(String, usize, usize),
    Namespace(String, usize, usize),
    Number(String, usize, usize),
    Operator(String, usize, usize),
    Parameter(String, usize, usize),
    Property(String, usize, usize),
    Regexp(String, usize, usize),
    String(String, usize, usize),
    Struct(String, usize, usize),
    Type(String, usize, usize),
    TypeParameter(String, usize, usize),
    Variable(String, usize, usize),
    Whitespace,

    //
    // Starting to put in specific things
    // to separate the tokens on nom
    // from the output tokens to make them
    // easier to test and change
    ListDash(String, usize, usize),
}

pub fn attributes(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, attr) = many1(terminated(attribute, opt(blank_line)))(source)?;
    Ok((
        source,
        attr.into_iter().flatten().collect::<Vec<NomToken>>(),
    ))
}

pub fn attribute(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, attr) = alt((key_value_attribute, boolean_attribute))(source)?;
    Ok((source, attr))
}

pub fn blank_line(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, _) = space0(source)?;
    let (source, _) = newline(source)?;
    Ok((source, vec![]))
}

pub fn boolean_attribute(source: Span) -> IResult<Span, Vec<NomToken>> {
    // dbg!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    // dbg!(source);
    // let (source, _) = multispace0(source)?;
    // let (source, _) = multispace0(source)?;
    // dbg!("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB");
    // dbg!(source);
    let (source, mut response) = dashes(source)?;
    // dbg!("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC");
    // dbg!(source);
    let (source, key_start) = position(source)?;
    // dbg!("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD");
    // dbg!(source);
    let (source, key_value) = is_not(":\n")(source)?;
    // dbg!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
    // dbg!(source);
    // let (source, _) = space0(source)?;
    // dbg!("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    // dbg!(source);
    // let (source, _) = newline(source)?;
    // dbg!("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
    // dbg!(source);
    let (source, key_end) = position(source)?;
    let key_attr = NomToken::Comment(
        key_value.to_string(),
        key_start.location_offset(),
        key_end.location_offset(),
    );
    response.push(key_attr);
    Ok((source, response))
}

pub fn code_type_section(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, mut response) = dashes(source)?;
    let (source, start) = position(source)?;
    let (source, name) = alt((tag("code"), tag("css"), tag("pre"), tag("script")))(source)?;
    let (source, end) = position(source)?;
    response.push(NomToken::Class(
        name.to_string(),
        start.location_offset(),
        end.location_offset(),
    ));
    let (source, _) = space0(source)?;
    let (source, _) = newline(source)?;
    let (source, attrs) = opt(separated_list0(tag("\n"), attribute))(source)?;
    if let Some(attrs) = attrs {
        response.append(&mut attrs.into_iter().flatten().collect::<Vec<NomToken>>());
    }
    let (source, _) = empty_line(source)?;
    let (source, mut paragraphs) = paragraphs(source)?;
    response.append(&mut paragraphs);
    Ok((source, response))
}

pub fn dashes(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, start) = position(source)?;
    let (source, _) = tag("--")(source)?;
    let (source, end) = position(source)?;
    let (source, _) = tag(" ")(source)?;
    Ok((
        source,
        vec![NomToken::Decorator(
            "--".to_string(),
            start.location_offset(),
            end.location_offset(),
        )],
    ))
}

pub fn empty_line(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, _) = tag("\n")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = tag("\n")(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, vec![NomToken::Whitespace]))
}

pub fn following_word_chars(source: Span) -> IResult<Span, Vec<NomToken>> {
    // Any character (including "<") that's not
    // a whitespace or a break
    let (source, start) = position(source)?;
    let (source, val) = is_not(" \n\t\r")(source)?;
    let (source, end) = position(source)?;
    Ok((
        source,
        vec![NomToken::String(
            val.to_string(),
            start.location_offset(),
            end.location_offset(),
        )],
    ))
}

pub fn initial_paragraph_word(source: Span) -> IResult<Span, Vec<NomToken>> {
    // get the first character of a word that
    // allows for a "<", but not two in a row
    let (source, response) = alt((word_without_leading_dash, word_without_leading_dash))(source)?;
    Ok((source, response))
}

pub fn initial_word_chars(source: Span) -> IResult<Span, Vec<NomToken>> {
    // get the first character of a word that
    // allows for a "<", but not two in a row
    let (source, response) = alt((non_lt_char, lt_with_non_lt_char))(source)?;
    Ok((source, response))
}

pub fn key_value_attribute(source: Span) -> IResult<Span, Vec<NomToken>> {
    // let (source, _) = multispace0(source)?;
    // let (source, _) = multispace0(source)?;
    let (source, mut response) = dashes(source)?;
    let (source, key_start) = position(source)?;
    let (source, key_value) = is_not(":\n")(source)?;
    let (source, key_end) = position(source)?;
    let key_attr = NomToken::Comment(
        key_value.to_string(),
        key_start.location_offset(),
        key_end.location_offset(),
    );
    response.push(key_attr);
    let (source, colon_start) = position(source)?;
    let (source, colon_value) = tag(":")(source)?;
    let (source, colon_end) = position(source)?;
    let colon = NomToken::Comment(
        colon_value.to_string(),
        colon_start.location_offset(),
        colon_end.location_offset(),
    );
    response.push(colon);
    let (source, _) = space0(source)?;
    let (source, val_start) = position(source)?;
    let (source, val_value) = is_not("\n")(source)?;
    let (source, val_end) = position(source)?;
    let val_attr = NomToken::Comment(
        val_value.to_string(),
        val_start.location_offset(),
        val_end.location_offset(),
    );
    response.push(val_attr);
    Ok((source, response))
}

pub fn list_item(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, start) = position(source)?;
    let (source, name) = tag("-")(source)?;
    let (source, end) = position(source)?;
    let dash = NomToken::ListDash(
        name.to_string(),
        start.location_offset(),
        end.location_offset(),
    );
    let (source, _) = space1(source)?;
    let mut response = vec![dash];
    let (source, mut paragraphs) = paragraphs(source)?;
    response.append(&mut paragraphs);
    Ok((source, response))
}

pub fn list_type_section(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, mut response) = dashes(source)?;
    let (source, start) = position(source)?;
    let (source, name) = alt((tag("list"), tag("notes"), tag("warnings")))(source)?;
    let (source, end) = position(source)?;
    response.push(NomToken::Class(
        name.to_string(),
        start.location_offset(),
        end.location_offset(),
    ));
    let (source, _) = space0(source)?;
    let (source, _) = newline(source)?;
    let (source, attrs) = opt(separated_list0(tag("\n"), attribute))(source)?;
    if let Some(attrs) = attrs {
        response.append(&mut attrs.into_iter().flatten().collect::<Vec<NomToken>>());
    }
    let (source, _) = empty_line(source)?;
    let (source, list_items) = opt(separated_list0(empty_line, list_item))(source)?;
    if let Some(list_items) = list_items {
        response.append(&mut list_items.into_iter().flatten().collect::<Vec<NomToken>>());
    }
    Ok((source, response))
}

pub fn lt_with_non_lt_char(source: Span) -> IResult<Span, Vec<NomToken>> {
    // A less than with a trailing non less than
    // character
    let (source, start1) = position(source)?;
    let (source, val1) = tag("<")(source)?;
    let (source, end1) = position(source)?;
    let mut response = vec![NomToken::String(
        val1.to_string(),
        start1.location_offset(),
        end1.location_offset(),
    )];
    let (source, mut second_char) = non_lt_char(source)?;
    response.append(&mut second_char);
    Ok((source, response))
}

pub fn metadata_type_section(source: Span) -> IResult<Span, Vec<NomToken>> {
    // dbg!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
    // dbg!(source);
    let (source, mut response) = dashes(source)?;
    let (source, start) = position(source)?;
    let (source, name) = alt((tag("metadata"), tag("categories"), tag("group")))(source)?;
    let (source, end) = position(source)?;
    response.push(NomToken::Class(
        name.to_string(),
        start.location_offset(),
        end.location_offset(),
    ));
    let (source, _) = blank_line(source)?;

    // let (source, _) = space0(source)?;
    // let (source, _) = newline(source)?;
    // let (source, attrs) = opt(separated_list0(tag("\n"), attribute))(source)?;
    let (source, attrs) = opt(attributes)(source)?;
    if let Some(mut attrs) = attrs {
        // response.append(&mut attrs.into_iter().flatten().collect::<Vec<NomToken>>());
        response.append(&mut attrs);
    }

    let (source, _) = opt(blank_line)(source)?;
    // dbg!("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD");
    // dbg!(source);
    Ok((source, response))
}

pub fn nom_parse(text: &str) -> IResult<Span, Vec<NomToken>> {
    let source = Span::new(text);
    let (source, response) = many1(section)(source)?;
    Ok((
        source,
        response.into_iter().flatten().collect::<Vec<NomToken>>(),
    ))
}

pub fn non_lt_char(source: Span) -> IResult<Span, Vec<NomToken>> {
    // Any character (including "<") that's not
    // a whitespace or a break
    let (source, start) = position(source)?;
    let (source, val) = none_of("< \n\t\r")(source)?;
    let (source, end) = position(source)?;
    Ok((
        source,
        vec![NomToken::String(
            val.to_string(),
            start.location_offset(),
            end.location_offset(),
        )],
    ))
}

pub fn paragraph(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, mut response) = initial_paragraph_word(source)?;
    let (source, _) = space0(source)?;
    let (source, items) = opt(separated_list1(wordbreak, word))(source)?;
    if let Some(items) = items {
        response.append(&mut items.into_iter().flatten().collect::<Vec<NomToken>>());
    }
    // items.into_iter().for_each(|mut i| response.append(&mut i));
    //response.append(&mut items.iter_mut().flatten().collect::<Vec<NomToken>>());
    // let mut collected_items = items.into_iter().flatten().collect::<Vec<NomToken>>();
    // response.append(&mut collected_items);
    Ok((
        source, // items.into_iter().flatten().collect::<Vec<NomToken>>(),
        response,
    ))
}

pub fn paragraph_type_section(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, mut response) = dashes(source)?;
    let (source, start) = position(source)?;
    let (source, name) = alt((
        tag("aside"),
        tag("blockquote"),
        tag("bookmark"),
        tag("footnote"),
        tag("h1"),
        tag("h2"),
        tag("h3"),
        tag("h4"),
        tag("h5"),
        tag("h6"),
        tag("hr"),
        tag("image"),
        tag("note"),
        tag("p"),
        tag("reference"),
        tag("subtitle"),
        tag("title"),
        tag("vimeo"),
        tag("warning"),
        tag("youtube"),
    ))(source)?;
    let (source, end) = position(source)?;
    response.push(NomToken::Class(
        name.to_string(),
        start.location_offset(),
        end.location_offset(),
    ));
    let (source, _) = space0(source)?;
    let (source, _) = newline(source)?;
    let (source, attrs) = opt(attributes)(source)?;
    if let Some(mut attrs) = attrs {
        // response.append(&mut attrs.into_iter().flatten().collect::<Vec<NomToken>>());
        response.append(&mut attrs);
    }
    let (source, _) = blank_line(source)?;
    let (source, mut paragraphs) = paragraphs(source)?;
    response.append(&mut paragraphs);
    Ok((source, response))
}

pub fn paragraphs(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, items) = separated_list1(empty_line, paragraph)(source)?;
    Ok((
        source,
        items.into_iter().flatten().collect::<Vec<NomToken>>(),
    ))
}

pub fn section(source: Span) -> IResult<Span, Vec<NomToken>> {
    // dbg!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE");
    // dbg!(source);
    let (source, response) = alt((
        paragraph_type_section,
        metadata_type_section,
        code_type_section,
        list_type_section,
    ))(source)?;
    let (source, _) = opt(empty_line)(source)?;
    Ok((source, response))
}

pub fn single_character_word(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, start) = position(source)?;
    let (source, the_char) = none_of(" \n")(source)?;
    let (source, end) = position(source)?;
    let response = vec![NomToken::String(
        the_char.to_string(),
        start.location_offset(),
        end.location_offset(),
    )];
    Ok((source, response))
}

pub fn single_newline(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, _) = tag("\n")(source)?;
    Ok((source, vec![NomToken::Whitespace]))
}

pub fn whitespace(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, _) = is_a(" \t")(source)?;
    Ok((source, vec![NomToken::Whitespace]))
}

pub fn word(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, response) = alt((word_base, single_character_word))(source)?;
    Ok((source, response))
}

pub fn word_base(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, mut response) = initial_word_chars(source)?;
    let (source, mut part_two) = following_word_chars(source)?;
    response.append(&mut part_two);
    Ok((source, response))
}

pub fn word_without_leading_dash(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, start) = position(source)?;
    let (source, part_one) = none_of("-")(source)?;
    let (source, end) = position(source)?;
    let mut response = vec![NomToken::String(
        part_one.to_string(),
        start.location_offset(),
        end.location_offset(),
    )];
    let (source, mut part_two) = following_word_chars(source)?;
    response.append(&mut part_two);
    Ok((source, response))
}

pub fn wordbreak(source: Span) -> IResult<Span, Vec<NomToken>> {
    let (source, _) = alt((single_newline, whitespace))(source)?;
    Ok((source, vec![NomToken::Whitespace]))
}

#[cfg(test)]

mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    pub fn test_attributes() {
        let source = Span::new("-- sierra\n-- bravo\n");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Comment("sierra".to_string(), 3, 9),
            NomToken::Decorator("--".to_string(), 10, 12),
            NomToken::Comment("bravo".to_string(), 13, 18),
        ];
        let right = attributes(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_blank_line() {
        let source = Span::new("  \n");
        let left = "";
        let right = blank_line(source).unwrap().0;
        assert_eq!(left.to_string(), right.fragment().to_string());
    }

    #[test]
    pub fn test_boolean_attribute() {
        let source = Span::new("-- sierra\n");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Comment("sierra".to_string(), 3, 9),
        ];
        let right = boolean_attribute(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_dashes() {
        let source = Span::new("-- ");
        let left = vec![NomToken::Decorator("--".to_string(), 0, 2)];
        let right = dashes(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_empty_line() {
        let source = Span::new("\n\n");
        let left = vec![NomToken::Whitespace];
        let right = empty_line(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_empty_line_including_whitespace() {
        let source = Span::new("\n  \n\n\n \n");
        let left = vec![NomToken::Whitespace];
        let right = empty_line(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_following_word_chars_test() {
        let source = Span::new("lfa");
        let left = vec![NomToken::String("lfa".to_string(), 0, 3)];
        let right = following_word_chars(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_initial_paragraph_word_via_just_text() {
        let source = Span::new("alfa ");
        let left = vec![
            NomToken::String("a".to_string(), 0, 1),
            NomToken::String("lfa".to_string(), 1, 4),
        ];
        let right = initial_paragraph_word(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_initial_word_chars_via_lt() {
        let source = Span::new("<f");
        let left = vec![
            NomToken::String("<".to_string(), 0, 1),
            NomToken::String("f".to_string(), 1, 2),
        ];
        let right = initial_word_chars(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_key_value_attribute() {
        let source = Span::new("-- alfa: bravo");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Comment("alfa".to_string(), 3, 7),
            NomToken::Comment(":".to_string(), 7, 8),
            NomToken::Comment("bravo".to_string(), 9, 14),
        ];
        let right = key_value_attribute(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_list_item() {
        let source = Span::new("- papa sierra");
        let left = vec![
            NomToken::ListDash("-".to_string(), 0, 1),
            NomToken::String("p".to_string(), 2, 3),
            NomToken::String("apa".to_string(), 3, 6),
            NomToken::String("s".to_string(), 7, 8),
            NomToken::String("ierra".to_string(), 8, 13),
        ];
        let right = list_item(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_nom_parse_basic() {
        // this is text, the span is created in the parser
        // for ease of use
        let source = "-- title\n\nsierra";
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("title".to_string(), 3, 8),
            NomToken::String("s".to_string(), 10, 11),
            NomToken::String("ierra".to_string(), 11, 16),
        ];
        let right = nom_parse(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_non_less_than_char_test() {
        let source = Span::new("a");
        let left = vec![NomToken::String("a".to_string(), 0, 1)];
        let right = non_lt_char(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_non_less_than_char_via_skip_lt() {
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
    pub fn test_lt_with_non_lt_chars() {
        let source = Span::new("<a");
        let left = vec![
            NomToken::String("<".to_string(), 0, 1),
            NomToken::String("a".to_string(), 1, 2),
        ];
        let right = lt_with_non_lt_char(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_paragraph() {
        let source = Span::new("echo <foxtrot hotel");
        let left = vec![
            NomToken::String("e".to_string(), 0, 1),
            NomToken::String("cho".to_string(), 1, 4),
            NomToken::String("<".to_string(), 5, 6),
            NomToken::String("f".to_string(), 6, 7),
            NomToken::String("oxtrot".to_string(), 7, 13),
            NomToken::String("h".to_string(), 14, 15),
            NomToken::String("otel".to_string(), 15, 19),
        ];
        let right = paragraph(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_paragraphs() {
        let source = Span::new("alfa\n\nbravo\n\ncharlie");
        let left = vec![
            NomToken::String("a".to_string(), 0, 1),
            NomToken::String("lfa".to_string(), 1, 4),
            NomToken::String("b".to_string(), 6, 7),
            NomToken::String("ravo".to_string(), 7, 11),
            NomToken::String("c".to_string(), 13, 14),
            NomToken::String("harlie".to_string(), 14, 20),
        ];
        let right = paragraphs(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_single_character_word() {
        let source = Span::new("<");
        let left = vec![NomToken::String("<".to_string(), 0, 1)];
        let right = single_character_word(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_single_newline() {
        let source = Span::new("\n");
        let left = vec![NomToken::Whitespace];
        let right = single_newline(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_whitespace() {
        let source = Span::new("  ");
        let left = vec![NomToken::Whitespace];
        let right = whitespace(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_word_via_lt() {
        let source = Span::new("<delta");
        let left = vec![
            NomToken::String("<".to_string(), 0, 1),
            NomToken::String("d".to_string(), 1, 2),
            NomToken::String("elta".to_string(), 2, 6),
        ];
        let right = word(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_word_without_leading_dash() {
        let source = Span::new("<delta");
        let left = vec![
            NomToken::String("<".to_string(), 0, 1),
            NomToken::String("delta".to_string(), 1, 6),
        ];
        let right = word_without_leading_dash(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_wordbreak() {
        let source = Span::new("\n");
        let left = vec![NomToken::Whitespace];
        let right = wordbreak(source).unwrap().1;
        assert_eq!(left, right);
    }

    // SECTION TYPES
    //

    #[test]
    pub fn test_paragraph_type_section() {
        let source = Span::new("-- h1\n\nAlfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("h1".to_string(), 3, 5),
            NomToken::String("A".to_string(), 7, 8),
            NomToken::String("lfa".to_string(), 8, 11),
        ];
        let right = paragraph_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    // SECTION TESTS

    #[test]
    // #[ignore]
    pub fn test_code_section() {
        let source = Span::new("-- code\n-- b\n\nAlfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("code".to_string(), 3, 7),
            NomToken::Decorator("--".to_string(), 8, 10),
            NomToken::Comment("b".to_string(), 11, 12),
            NomToken::String("A".to_string(), 14, 15),
            NomToken::String("lfa".to_string(), 15, 18),
        ];
        let right = code_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_section() {
        let source = Span::new("-- h1\n\nAlfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("h1".to_string(), 3, 5),
            NomToken::String("A".to_string(), 7, 8),
            NomToken::String("lfa".to_string(), 8, 11),
        ];
        let right = section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    pub fn test_list_type_section() {
        let source = Span::new("-- notes\n-- b\n\n- Alfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("notes".to_string(), 3, 8),
            NomToken::Decorator("--".to_string(), 9, 11),
            NomToken::Comment("b".to_string(), 12, 13),
            NomToken::ListDash("-".to_string(), 15, 16),
            NomToken::String("A".to_string(), 17, 18),
            NomToken::String("lfa".to_string(), 18, 21),
        ];
        let right = list_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_metadata_type_section_basic() {
        let source = Span::new("-- metadata\n-- id: asdf");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("metadata".to_string(), 3, 11),
            NomToken::Decorator("--".to_string(), 12, 14),
            NomToken::Comment("id".to_string(), 15, 17),
            NomToken::Comment(":".to_string(), 17, 18),
            NomToken::Comment("asdf".to_string(), 19, 23),
        ];
        let right = metadata_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_metadata_type_sections_next_to_each_other() {
        let source = "-- categories\n-- Rust\n\n-- metadata\n-- id: tango";
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("categories".to_string(), 3, 13),
            NomToken::Decorator("--".to_string(), 14, 16),
            NomToken::Comment("Rust".to_string(), 17, 21),
            NomToken::Decorator("--".to_string(), 23, 25),
            NomToken::Class("metadata".to_string(), 26, 34),
            NomToken::Decorator("--".to_string(), 35, 37),
            NomToken::Comment("id".to_string(), 38, 40),
            NomToken::Comment(":".to_string(), 40, 41),
            NomToken::Comment("tango".to_string(), 42, 47),
        ];
        let right = nom_parse(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_h1_section() {
        let source = Span::new("-- h1\n\nAlfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("h1".to_string(), 3, 5),
            NomToken::String("A".to_string(), 7, 8),
            NomToken::String("lfa".to_string(), 8, 11),
        ];
        let right = paragraph_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    pub fn test_title_section() {
        let source = Span::new("-- title\n\nAlfa\n\nBravo");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("title".to_string(), 3, 8),
            NomToken::String("A".to_string(), 10, 11),
            NomToken::String("lfa".to_string(), 11, 14),
            NomToken::String("B".to_string(), 16, 17),
            NomToken::String("ravo".to_string(), 17, 21),
        ];
        let right = paragraph_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    pub fn test_title_with_key_value_attributes() {
        let source = Span::new("-- title\n-- autofocus\n-- delta: echo\n\nAlfa\n\nBravo");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("title".to_string(), 3, 8),
            NomToken::Decorator("--".to_string(), 9, 11),
            NomToken::Comment("autofocus".to_string(), 12, 21),
            NomToken::Decorator("--".to_string(), 22, 24),
            NomToken::Comment("delta".to_string(), 25, 30),
            NomToken::Comment(":".to_string(), 30, 31),
            NomToken::Comment("echo".to_string(), 32, 36),
            NomToken::String("A".to_string(), 38, 39),
            NomToken::String("lfa".to_string(), 39, 42),
            NomToken::String("B".to_string(), 44, 45),
            NomToken::String("ravo".to_string(), 45, 49),
        ];
        let right = paragraph_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    pub fn test_title_with_boolean_attributes() {
        let source = Span::new("-- title\n-- b\n\nAlfa");
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("title".to_string(), 3, 8),
            NomToken::Decorator("--".to_string(), 9, 11),
            NomToken::Comment("b".to_string(), 12, 13),
            NomToken::String("A".to_string(), 15, 16),
            NomToken::String("lfa".to_string(), 16, 19),
        ];
        let right = paragraph_type_section(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    // #[ignore]
    pub fn test_integration_basic() {
        let source = "-- title\n\nAlfa\n\n-- h2\n\nBravo a b";
        let left = vec![
            NomToken::Decorator("--".to_string(), 0, 2),
            NomToken::Class("title".to_string(), 3, 8),
            NomToken::String("A".to_string(), 10, 11),
            NomToken::String("lfa".to_string(), 11, 14),
            NomToken::Decorator("--".to_string(), 16, 18),
            NomToken::Class("h2".to_string(), 19, 21),
            NomToken::String("B".to_string(), 23, 24),
            NomToken::String("ravo".to_string(), 24, 28),
            NomToken::String("a".to_string(), 29, 30),
            NomToken::String("b".to_string(), 31, 32),
        ];
        let right = nom_parse(source).unwrap().1;
        assert_eq!(left, right);
    }

    //
}
