use std::borrow::Cow;

use nom::{branch::alt, combinator::map};

use super::parser::{Error, Input, line_with_leader, impl_from_str};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Heading<'s> {
    H1(Cow<'s, str>),
    H2(Cow<'s, str>),
    H3(Cow<'s, str>),
}

impl Heading<'_> {
    pub fn into_static(self) -> Heading<'static> {
        match self {
            Self::H1(s) => Heading::H1(s.into_owned().into()),
            Self::H2(s) => Heading::H2(s.into_owned().into()),
            Self::H3(s) => Heading::H3(s.into_owned().into()),
        }
    }
}

impl<'s> Heading<'s> {
    pub(crate) fn parse(input: Input<'s>) -> nom::IResult<Input<'_>, Self, Error<'s>> {
        alt((
            map(line_with_leader("###"), Heading::H3),
            map(line_with_leader("##"), Heading::H2),
            map(line_with_leader("#"), Heading::H1),
        ))(input)
    }
}

impl_from_str!(Heading);

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#Test", Heading::H1(Cow::Borrowed("Test")))]
    fn test_valid_from_str(input: &str, expected: Heading<'static>) {
        let value: Heading = input.parse().expect("should parse successfully");
        assert_eq!(value, expected);
    }
}
