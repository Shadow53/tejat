use std::borrow::Cow;

use nom::{combinator::{recognize, not, opt, map, peek, eof}, multi::{many1_count, many_till}, character::complete::{space1, space0, anychar}, sequence::{tuple, preceded}, branch::alt};
use url::Url;

use super::{parser::{Input, IResult, better_tag, optional_str_until_newline, impl_from_str}, Error};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum LinkTarget<'s> {
    Absolute(Url),
    Relative(Cow<'s, str>),
}

impl<'s> LinkTarget<'s> {
    pub(crate) fn parse(input: Input<'s>) -> IResult<Self> {
        map(
            recognize(many_till(anychar, peek(alt((space1, eof))))),
            |s: Input<'_>| { 
                s.parse::<Url>()
                    .map(LinkTarget::Absolute)
                    .unwrap_or_else(|_| LinkTarget::Relative(Cow::Borrowed(*s.fragment())))
            },
        )(input)
    }

    fn into_static(self) -> LinkTarget<'static> {
        match self {
            Self::Absolute(url) => LinkTarget::Absolute(url),
            Self::Relative(path) => LinkTarget::Relative(Cow::Owned(path.into_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Link<'s> {
    target: LinkTarget<'s>,
    text: Option<Cow<'s, str>>,
}

impl<'s> Link<'s> {
    pub fn new<S>(url: S, text: Option<S>) -> Result<Self, Error<'s>>
        where S: Into<Cow<'s, str>>,
    {
        let url = url.into();
        let text = text.map(Into::into);
        let target: LinkTarget = url.parse()?;
        Ok(Self {
            target,
            text,
        })
    }

    pub(crate) fn parse(input: Input<'s>) -> IResult<Self> {
        map(
            tuple((
                better_tag("=>"),
                space0,
                LinkTarget::parse,
                alt((
                    preceded(space1, optional_str_until_newline),
                    map(space0, |_| None),
                )),
            )),
            |(_, _, target, text)| {
                Link { target, text }
            }
        )(input)
    }

    pub fn into_static(self) -> Link<'static> {
        Link {
            target: self.target.into_static(),
            text: self.text.map(|s| Cow::Owned(s.into_owned())),
        }
    }
}

impl_from_str!(Link);
impl_from_str!(LinkTarget);

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "=> /relative/url  ",
        Link {
            target: LinkTarget::Relative(Cow::Borrowed("/relative/url")),
            text: None,
        }
    )]
    fn test_from_str_valid(input: &str, expected: Link<'static>) {
        let link: Link = input.parse().expect("should be a valid link line");
        assert_eq!(link, expected);
    }
}
