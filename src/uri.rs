use std::{str::FromStr, fmt};
use url::Url;

const SCHEME_GEMINI: &str = "gemini";

pub enum UrlParseError {
    InvalidUrl(url::ParseError),
    /// Contains the non-gemini scheme as [`String`].
    NotGeminiScheme(Url),
    HasUserInfo(Url),
    MissingAuthority(Url),
    MissingHost(Url),
}

impl From<url::ParseError> for UrlParseError {
    fn from(error: url::ParseError) -> Self {
        Self::InvalidUrl(error)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeminiUrl(Url);

impl TryFrom<Url> for GeminiUrl {
    type Error = UrlParseError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        if url.scheme() != SCHEME_GEMINI {
            Err(UrlParseError::NotGeminiScheme(url))
        } else if url.has_authority() {
            Err(UrlParseError::MissingAuthority(url))
        } else if url.has_host() {
            Err(UrlParseError::MissingHost(url))
        } else if url.password().is_some() || !url.username().is_empty() {
            Err(UrlParseError::HasUserInfo(url))
        } else {
            Ok(Self(url))
        }
    }
}

impl FromStr for GeminiUrl {
    type Err = UrlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Url>().map(GeminiUrl::try_from)?
    }
}

impl fmt::Display for GeminiUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
