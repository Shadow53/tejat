use std::str::FromStr;

use nom::{sequence::{tuple, pair}, bytes::complete::tag, combinator::{recognize, not, eof, opt, map, map_res, all_consuming}, multi::{many0_count, many1_count, many0}, character::complete::{line_ending, space0, space1}, branch::alt};
use url::Url;

pub enum Heading<'s> {
    H1(&'s str),
    H2(&'s str),
    H3(&'s str),
}

pub enum RawLine<'s> {
    Blockquote(&'s str),
    Heading(Heading<'s>),
    Link(Link<'s>),
    ListItem(&'s str),
    Preformatted(Preformatted<'s>),
    Text(&'s str),
}

pub struct Link<'s> {
    pub target: Url,
    pub text: Option<&'s str>,
}

pub struct Preformatted<'s> {
    pub alt_text: Option<&'s str>,
    pub text: &'s str,
}

pub fn parse_lines(input: &str) -> nom::IResult<&str, Vec<RawLine>> {
    all_consuming(many0(alt((
        parse_preformatted,
        parse_heading,
        parse_link,
        parse_list_item,
        parse_blockquote,
        parse_text,
    ))))(input)
}

fn line_end(input: &str) -> nom::IResult<&str, &str> {
    alt((line_ending, eof))(input)
}

fn str_until_newline(input: &str) -> nom::IResult<&str, &str> {
    pair(
        recognize(many0_count(not(line_ending))),
        line_end,
    )(input).map(|(leftover, (text, _))| {
        (leftover, text)
    })
}

fn optional_str_until_newline(input: &str) -> nom::IResult<&str, Option<&str>> {
    map(str_until_newline, |s| (!s.is_empty()).then(|| s))(input)
}

fn parse_blockquote(input: &str) -> nom::IResult<&str, RawLine> {
    map(line_with_leader(">"), RawLine::Blockquote)(input)
}

fn line_with_leader(lead: &'static str) -> impl Fn(&str) -> nom::IResult<&str, &str> {
    move |input| {
        tuple((
            tag(lead),
            space0,
            str_until_newline,
        ))(input).map(|(leftover, (_, _, text))| {
            (leftover, text)
        })
    }
}

fn parse_heading(input: &str) -> nom::IResult<&str, RawLine> {
    alt((
        map(map(line_with_leader("###"), Heading::H3), RawLine::Heading),
        map(map(line_with_leader("##"), Heading::H2), RawLine::Heading),
        map(map(line_with_leader("#"), Heading::H1), RawLine::Heading),
    ))(input)
}

fn parse_url(input: &str) -> nom::IResult<&str, Url> {
    map_res(
        recognize(many1_count(not(space1))),
        Url::from_str,
    )(input)
}

fn parse_link(input: &str) -> nom::IResult<&str, RawLine> {
    tuple((
        tag("=>"),
        space0,
        parse_url,
        opt(pair(space1, optional_str_until_newline)),
    ))(input).map(|(leftover, (_, _, target, opt_text))| {
        let text = opt_text.and_then(|(_, text)| text);
        (leftover, RawLine::Link(Link { target, text }))
    })
}

fn parse_list_item(input: &str) -> nom::IResult<&str, RawLine> {
    map(line_with_leader("* "), RawLine::ListItem)(input)
}

fn parse_preformatted(input: &str) -> nom::IResult<&str, RawLine> {
    tuple((
        tag("```"),
        optional_str_until_newline,
        recognize(many0_count(not(tuple((line_ending, tag("```"), line_ending))))),
        tag("```"),
        line_end
    ))(input).map(|(leftover, (_, alt_text, text, _, _))| {
        (leftover, RawLine::Preformatted(Preformatted { alt_text, text }))
    })
}

fn parse_text(input: &str) -> nom::IResult<&str, RawLine> {
    map(str_until_newline, RawLine::Text)(input)
}
