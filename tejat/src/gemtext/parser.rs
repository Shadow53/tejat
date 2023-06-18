use nom::{sequence::{preceded, pair, terminated}, bytes::complete::tag, character::complete::{space0, line_ending, anychar}, branch::alt, combinator::{eof, recognize, map, peek}, multi::many_till, error::{ParseError, ContextError, context, FromExternalError}, Parser};
use nom_locate::LocatedSpan;
use std::borrow::Cow;

const SNIPPET_LEN: usize = 15;
pub(crate) type Input<'s> = LocatedSpan<&'s str>;
pub(crate) type IResult<'s, T> = nom::IResult<Input<'s>, T, Error<'s>>;

#[derive(Debug, thiserror::Error)]
#[error("error at line {line}, column {column} ({snippet:?}): {kind}")]
pub struct Error<'s> {
    line: u32,
    column: usize,
    snippet: Cow<'s, str>,
    kind: ErrorKind,
    prev: Option<Box<Self>>,
}

impl<'s> Error<'s> {
    #[inline]
    fn inner_new(input: Input<'s>, kind: ErrorKind, prev: Option<Box<Self>>) -> Self {
        Self {
            line: input.location_line(),
            column: input.get_utf8_column(),
            snippet: Cow::Borrowed(input.fragment().get(0..SNIPPET_LEN).unwrap_or_else(|| input.fragment())),
            kind,
            prev,
        }
    }

    pub fn new(input: Input<'s>, kind: ErrorKind) -> Self {
        Self::inner_new(input, kind, None)
    }

    pub fn with_previous(input: Input<'s>, kind: ErrorKind, prev: Self) -> Self {
        Self::inner_new(input, kind, Some(Box::new(prev)))
    }

    pub fn into_static(self) -> Error<'static> {
        Error {
            snippet: Cow::Owned(self.snippet.into_owned()),
            prev: self.prev.map(|prev| Box::new(prev.into_static())),
            .. self
        }
    }
}

impl<'s> ParseError<Input<'s>> for Error<'s> {
    fn from_error_kind(input: Input<'s>, kind: nom::error::ErrorKind) -> Self {
        Self::new(input, ErrorKind::Internal(kind))
    }

    fn append(_input: Input<'s>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        // TODO: stack errors?
        other
    }
}

impl<'s> ContextError<Input<'s>> for Error<'s> {
    fn add_context(input: Input<'s>, ctx: &'static str, other: Self) -> Self {
        // TODO: previous error
        Self::with_previous(input, ErrorKind::Context(ctx), other)
    }
}

impl<'s> FromExternalError<Input<'s>, url::ParseError> for Error<'s> {
    fn from_external_error(input: Input<'s>, _kind: nom::error::ErrorKind, e: url::ParseError) -> Self {
        Self::new(input, ErrorKind::InvalidUrl(e))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("expected {0}")]
    Context(&'static str),
    #[error("expected end of input")]
    ExpectedEOF,
    #[error("expected the end of the line")]
    ExpectedLineEnd,
    #[error("expected one or more whitespace characters")]
    ExpectedWhitespace,
    #[error("expected {0:?}")]
    ExpectedStr(&'static str),
    #[error("internal parser error: {0:?}")]
    Internal(nom::error::ErrorKind),
    #[error("invalid URL: {0}")]
    InvalidUrl(url::ParseError),
}

pub(crate) fn better_all_consuming<'s, O, F>(mut parser: F) -> impl FnMut(Input<'s>) -> IResult<'s, O>
    where F: Parser<Input<'s>, O, Error<'s>>
{
    move |input| {
        let (leftover, parsed) = parser.parse(input)?;
        if leftover.is_empty() {
            Ok((leftover, parsed))
        } else {
            Err(nom::Err::Error(Error::new(leftover, ErrorKind::ExpectedEOF)))
        }
    }
}

pub(crate) fn repeated_all_consuming<'s, O, F>(parser: F) -> impl FnMut(Input<'s>) -> IResult<'s, Vec<O>>
    where F: Parser<Input<'s>, O, Error<'s>>
{
    let mut parser = many_till(parser, eof);
    move |input| {
        let (leftover, (parsed, _eof)) = parser.parse(input)?;
        if leftover.is_empty() {
            Ok((leftover, parsed))
        } else {
            Err(nom::Err::Error(Error::new(leftover, ErrorKind::ExpectedEOF)))
        }
    }
}

pub(crate) fn better_tag(literal: &'static str) -> impl Fn(Input<'_>) -> IResult<'_, Input<'_>> {
    move |input| {
        tag(literal)(input).map_err(|err| {
            err.map(|error: nom::error::Error::<Input<'_>>| {
                Error::new(error.input, ErrorKind::ExpectedStr(literal))
            })
        })
    }
}

pub(crate) fn line_end(input: Input<'_>) -> IResult<Input<'_>> {
    context(
        "a line ending or EOF",
        alt((line_ending, eof))
    )(input)
}

pub(crate) fn str_until_newline(input: Input<'_>) -> IResult<Cow<'_, str>> {
    let str_parser = context(
        "a line of text",
        recognize(many_till(anychar, peek(line_end))),
    );

    map(
        terminated(
            str_parser,
            line_end,
        ),
        |text| {
            Cow::Borrowed(*text.fragment())
        }
    )(input)
}

pub(crate) fn optional_str_until_newline(input: Input<'_>) -> IResult<Option<Cow<'_, str>>> {
    map(str_until_newline, |s| (!s.is_empty()).then_some(s))(input)
}

pub(crate) fn line_with_leader(lead: &'static str) -> impl Fn(Input<'_>) -> IResult<Cow<'_, str>> {
    move |input| {
        preceded(
            pair(
                better_tag(lead),
                space0,
            ),
            str_until_newline,
        )(input)
    }
}

mod macros {
    macro_rules! impl_from_str {
        ($name: ident) => {
            impl ::std::str::FromStr for $name<'static> {
                type Err = $crate::gemtext::Error<'static>;
                fn from_str(input: &str) -> Result<Self, Self::Err> {
                    let input = $crate::gemtext::parser::Input::new(input);
                    ::nom::Finish::finish($name::parse(input))
                        .map_err($crate::gemtext::Error::into_static)
                        .map(|(_, value)| value)
                        .map($name::into_static)
                }
            }
        }
    }

    pub(crate) use impl_from_str;
}

pub(super) use macros::impl_from_str;
