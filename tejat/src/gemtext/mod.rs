mod heading;
mod link;
mod macros;
mod parser;

use parser::{impl_from_str, Input, IResult};
pub use parser::{Error, ErrorKind};

use std::borrow::Cow;

use nom::{Finish, sequence::{tuple, pair}, bytes::complete::tag, combinator::{recognize, not, opt, map, map_res}, multi::{many1_count, many_till}, character::complete::{line_ending, space0, space1, anychar}, branch::alt};

pub use heading::Heading;
pub use link::Link;

use self::parser::{line_with_leader, str_until_newline, line_end, better_tag, optional_str_until_newline, repeated_all_consuming};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RawLine<'s> {
    Blockquote(Cow<'s, str>),
    Heading(Heading<'s>),
    Link(Link<'s>),
    ListItem(Cow<'s, str>),
    Preformatted(Preformatted<'s>),
    Text(Cow<'s, str>),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Preformatted<'s> {
    pub alt_text: Option<Cow<'s, str>>,
    pub text: Cow<'s, str>,
}

pub fn parse_lines(input: &str) -> Result<Vec<RawLine>, Error<'_>> {
    let input = Input::new(input);
    repeated_all_consuming(alt((
        parse_preformatted,
        map(Heading::parse, RawLine::Heading),
        map(Link::parse, RawLine::Link),
        parse_list_item,
        parse_blockquote,
        parse_text,
    )))(input)
        .finish()
        .map(|(_, value)| value)
}

fn parse_blockquote(input: Input<'_>) -> IResult<RawLine<'_>> {
    map(line_with_leader(">"), RawLine::Blockquote)(input)
}

fn parse_list_item(input: Input<'_>) -> IResult<RawLine<'_>> {
    map(line_with_leader("* "), RawLine::ListItem)(input)
}

fn parse_preformatted(input: Input<'_>) -> IResult<RawLine<'_>> {
    map(
        tuple((
            better_tag("```"),
            optional_str_until_newline,
            recognize(many_till(anychar, tuple((line_ending, tag("```"), line_ending)))),
            tag("```"),
            line_end
        )),
        |(_, alt_text, text, _, _)| {
            let text = Cow::Borrowed(*text.fragment());
            RawLine::Preformatted(Preformatted { alt_text, text })
        }
    )(input)
}

fn parse_text(input: Input<'_>) -> IResult<RawLine<'_>> {
    map(str_until_newline, RawLine::Text)(input)
}
