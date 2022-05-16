use crate::mime::Mime;
use crate::uri::URI;
use std::{fmt, ops::Deref, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Expected a status code, found a non-number instead.
    InvalidCode(String),
    /// Expected a valid status code, found an unknown number instead.
    UnknownCode(u8),
    /// Expected a valid status code, found one for a different status type.
    WrongCodeForStatus(u8),
    /// Not a valid status string.
    InvalidStatus(String),
    /// Not a valid meta string for this status code.
    InvalidMeta(String),
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!("infallible errors should never occur")
    }
}

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Code(u8);

impl Deref for Code {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Code> for u8 {
    fn from(code: Code) -> u8 {
        *code
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for Code {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(Error::UnknownCode(value))
        }
    }
}

impl FromStr for Code {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Status codes are always two digits.
        // This disallows any shenanigans with signs and such.
        if s.len() != 2 {
            return Err(Error::InvalidCode(s.to_string()))
        }

        s.parse::<u8>().map(Code::try_from).map_err(|_| {
            Error::InvalidCode(s.to_string())
        })?
    }
}

impl Code {
    pub const INPUT: Self = Self(10);
    pub const SENSITIVE_INPUT: Self = Self(11);
    pub const SUCCESS: Self = Self(20);
    pub const TEMPORARY_REDIRECT: Self = Self(30);
    pub const PERMANENT_REDIRECT: Self = Self(31);
    pub const TEMPORARY_FAILURE: Self = Self(40);
    pub const SERVER_UNAVAILABLE: Self = Self(41);
    pub const CGI_ERROR: Self = Self(42);
    pub const PROXY_ERROR: Self = Self(43);
    pub const SLOW_DOWN: Self = Self(44);
    pub const PERMANENT_FAILURE: Self = Self(50);
    pub const NOT_FOUND: Self = Self(51);
    pub const GONE: Self = Self(52);
    pub const PROXY_REQUEST_REFUSED: Self = Self(53);
    pub const BAD_REQUEST: Self = Self(59);
    pub const CLIENT_CERTIFICATE_REQUIRED: Self = Self(60);
    pub const CERTIFICATE_NOT_AUTHORISED: Self = Self(61);
    pub const CERTIFICATE_NOT_VALID: Self = Self(62);

    const ALL_CODES: [Self; 18] = [
        Code::INPUT,
        Code::SENSITIVE_INPUT,
        Code::SUCCESS,
        Code::TEMPORARY_REDIRECT,
        Code::PERMANENT_REDIRECT,
        Code::TEMPORARY_FAILURE,
        Code::SERVER_UNAVAILABLE,
        Code::CGI_ERROR,
        Code::PROXY_ERROR,
        Code::SLOW_DOWN,
        Code::PERMANENT_FAILURE,
        Code::NOT_FOUND,
        Code::GONE,
        Code::PROXY_REQUEST_REFUSED,
        Code::BAD_REQUEST,
        Code::CLIENT_CERTIFICATE_REQUIRED,
        Code::CERTIFICATE_NOT_AUTHORISED,
        Code::CERTIFICATE_NOT_VALID,
    ];

    fn is_valid(code: u8) -> bool {
        let code = Code(code);
        Code::ALL_CODES.binary_search(&code).is_ok()
    }
}

macro_rules! impl_code {
    ($name: ident { $($code: expr => $variant: ident { $meta: ident : $meta_type: ty },)* }) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $($variant { $meta: $meta_type },)*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant { $meta } => write!(f, "{} {}", $code, $meta),)*
                }
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let s = s.strip_suffix("\r\n").unwrap_or(s);
                let (code_str, meta) = s.split_once(' ')
                    .ok_or_else(|| Error::InvalidStatus(s.to_string()))?;
                let code: Code = code_str.parse()?;
                $(
                if code == $code {
                    let $meta = meta.parse::<$meta_type>().map_err(|_| Error::InvalidMeta(meta.to_string()))?;
                    return Ok($name::$variant { $meta });
                }
                )*
                Err(Error::WrongCodeForStatus(*code))
            }
        }

        impl $name {
            pub fn code(&self) -> Code {
                match self {
                    $(Self::$variant { .. } => $code,)*
                }
            }
        }
    }
}

impl_code! {
    Input {
        Code::INPUT => Normal { prompt: String },
        Code::SENSITIVE_INPUT => Sensitive { prompt: String },
    }
}

impl Input {
    pub fn prompt(&self) -> &str {
        match self {
            Self::Normal { prompt } | Self::Sensitive { prompt } => prompt,
        }
    }

    pub fn is_sensitive(&self) -> bool {
        matches!(self, Self::Sensitive { .. })
    }
}

impl_code! {
    Success {
        Code::SUCCESS => Normal { mime: Mime },
    }
}

impl Success {
    pub fn mime(&self) -> &Mime {
        match self {
            Self::Normal { mime } => mime,
        }
    }
}

impl_code! {
    Redirect {
        Code::TEMPORARY_REDIRECT => Temporary { target: URI },
        Code::PERMANENT_REDIRECT => Permanent { target: URI },
    }
}

impl Redirect {
    pub fn target(&self) -> &URI {
        match self {
            Self::Temporary { target } | Self::Permanent { target } => target
        }
    }

    pub fn is_temporary(&self) -> bool {
        matches!(self, Self::Temporary { .. })
    }

    pub fn is_permanent(&self) -> bool {
        matches!(self, Self::Permanent { .. })
    }
}

impl_code! {
    TemporaryFailure {
        Code::TEMPORARY_FAILURE => Generic { message: String },
        Code::SERVER_UNAVAILABLE => ServerUnavailable { message: String },
        Code::CGI_ERROR => CgiError { message: String },
        Code::PROXY_ERROR => ProxyError { message: String },
        Code::SLOW_DOWN => SlowDown { wait_secs: u16 },
    }
}

impl_code! {
    PermanentFailure {
        Code::PERMANENT_FAILURE => Generic { message: String },
        Code::NOT_FOUND => NotFound { message: String },
        Code::GONE => Gone { message: String },
        Code::PROXY_REQUEST_REFUSED => ProxyRequestRefused { message: String },
        Code::BAD_REQUEST => BadRequest { message: String },
    }
}

impl_code! {
    ClientCertificateRequired {
        Code::CLIENT_CERTIFICATE_REQUIRED => Required { message: String },
        Code::CERTIFICATE_NOT_AUTHORISED => NotAuthorised { message: String },
        Code::CERTIFICATE_NOT_VALID => NotValid { message: String },
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Status {
    Input(Input),
    Success(Success),
    Redirect(Redirect),
    TemporaryFailure(TemporaryFailure),
    PermanentFailure(PermanentFailure),
    ClientCertificateRequired(ClientCertificateRequired),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(inner) => write!(f, "{}\r\n", inner),
            Self::Success(inner) => write!(f, "{}\r\n", inner),
            Self::Redirect(inner) => write!(f, "{}\r\n", inner),
            Self::TemporaryFailure(inner) => write!(f, "{}\r\n", inner),
            Self::PermanentFailure(inner) => write!(f, "{}\r\n", inner),
            Self::ClientCertificateRequired(inner) => write!(f, "{}\r\n", inner),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code_str, _) = s.split_once(' ').ok_or_else(|| {
            Error::InvalidStatus(s.to_string())
        })?;

        let code: Code = code_str.parse()?;

        match *code / 10 * 10 {
            10 => s.parse::<Input>().map(Self::Input),
            20 => s.parse::<Success>().map(Self::Success),
            30 => s.parse::<Redirect>().map(Self::Redirect),
            40 => s.parse::<TemporaryFailure>().map(Self::TemporaryFailure),
            50 => s.parse::<PermanentFailure>().map(Self::PermanentFailure),
            60 => s.parse::<ClientCertificateRequired>().map(Self::ClientCertificateRequired),
            _ => Err(Error::UnknownCode(code.into())),
        }
    }
}

impl Status {
    pub fn code(&self) -> Code {
        match self {
            Self::Input(inner) => inner.code(),
            Self::Success(inner) => inner.code(),
            Self::Redirect(inner) => inner.code(),
            Self::TemporaryFailure(inner) => inner.code(),
            Self::PermanentFailure(inner) => inner.code(),
            Self::ClientCertificateRequired(inner) => inner.code(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    mod code {
        use super::*;

        const RAW_CODES: [u8; 18] = [10, 11, 20, 30, 31, 40, 41, 42, 43, 44, 50, 51, 52, 53, 59, 60, 61, 62];
        const INVALID_RAW: u8 = 69;

        #[test]
        fn test_list_of_codes_is_sorted() {
            let is_sorted = Code::ALL_CODES.windows(2).all(|win| win[0] < win[1]);
            assert!(is_sorted, "list of codes must be sorted for FromStr to work properly");
        }

        #[test]
        fn test_code_into_u8() {
            assert_eq!(RAW_CODES.len(), Code::ALL_CODES.len(), "not all valid codes are accounted for");
            for (raw, code) in RAW_CODES.iter().copied().zip(Code::ALL_CODES.iter().copied()) {
                assert_eq!(raw, u8::from(code));
            }
        }

        mod from_str {
            use super::*;

            fn invalid_str(s: String) {
                let err = s.parse::<Code>().unwrap_err();
                if let Error::InvalidCode(err_str) = err {
                    assert_eq!(s, err_str);
                } else {
                    panic!("expected Error::InvalidCode, got {:?}", err);
                }
            }

            fn invalid_code(code: u8, s: String) {
                let err = s.parse::<Code>().unwrap_err();
                if let Error::UnknownCode(err_code) = err {
                    assert_eq!(err_code, code);
                } else {
                    panic!("expected Error::UnknownCode, got {:?}", err);
                }
            }

            #[test]
            fn test_valid_codes() {
                for code in Code::ALL_CODES {
                    let new_code: Code = u8::from(code).to_string().parse().unwrap();
                    assert_eq!(code, new_code);
                }
            }

            #[test]
            fn test_leading_space_is_error() {
                invalid_str(format!(" {}", RAW_CODES[0]));
            }

            #[test]
            fn test_trailing_space_is_error() {
                invalid_str(format!("{} ", RAW_CODES[0]));
            }

            #[test]
            fn test_invalid_code() {
                invalid_code(INVALID_RAW, INVALID_RAW.to_string());
            }

            #[test]
            fn test_more_than_two_digits_not_allowed() {
                invalid_str(format!("0{}", RAW_CODES[0]));
                invalid_str(format!("000{}", RAW_CODES[0]));
            }

            #[test]
            fn test_signs_not_allowed() {
                invalid_str(format!("+{}", RAW_CODES[0]));
                invalid_str(format!("-{}", RAW_CODES[0]));
            }

            #[test]
            fn test_binary_not_allowed() {
                // Binary without leading 0b could be, e.g. `10 INPUT`
                invalid_str(format!("{:#b}", RAW_CODES[0]));
            }

            #[test]
            fn test_octal_not_allowed() {
                // Octal without leading 0o could be, e.g. `10 INPUT`
                invalid_str(format!("{:#o}", RAW_CODES[0]));
            }

            #[test]
            fn test_hexadecimal_not_allowed() {
                // Use 20 SUCCESS because it is two hex digits: 0x14
                let code = *Code::SUCCESS;
                invalid_code(14, format!("{:x}", code));
                invalid_code(14, format!("{:X}", code));
                invalid_str(format!("{:#x}", code));
                invalid_str(format!("{:#X}", code));
            }

            #[test]
            fn test_scientific_notation_not_allowed() {
                invalid_str(format!("{:e}", RAW_CODES[0]));
                invalid_str(format!("{:E}", RAW_CODES[0]));
            }
        }

        mod try_from_u8 {
            use super::*;

            #[test]
            fn test_valid_codes() {
                for code in Code::ALL_CODES {
                    let new_code = Code::try_from(u8::from(code)).unwrap();
                    assert_eq!(code, new_code);
                }
            }
        }
    }

    macro_rules! test_status_enum {
        (
            module: $module: ident,
            name: $name: ident,
            valid: {
                $($valid_str: literal => ($code: expr, $expected: expr)),*
            },
            invalid: {
                $($invalid_str: literal => $expected_err: expr),*
            }
        ) => {
            mod $module {
                use super::*;

                test_status_enum! {
                    name: Status,
                    valid: {
                        $($valid_str => ($code, Status::$name($expected))),*
                    },
                    invalid: {
                        $($invalid_str => $expected_err),*
                    },
                    wrapped: true
                }
            }

            paste! {
            mod [< $module _status >] {
                use super::*;

                test_status_enum! {
                    name: $name,
                    valid: {
                        $($valid_str => ($code, $expected)),*
                    },
                    invalid: {
                        $($invalid_str => $expected_err),*
                    },
                    wrapped: false
                }
            }
            }
        };
        (
            name: $name: ident,
            valid: {
                $($valid_str: literal => ($code: expr, $expected: expr)),*
            },
            invalid: {
                $($invalid_str: literal => $expected_err: expr),*
            },
            wrapped: $wrapped: literal
        ) => {
            #[test]
            fn test_valid_from_str() {
                $(
                    let result = $valid_str.parse::<$name>().unwrap_or_else(|err| {
                        panic!("expected \"{}\" to parse, got error: {:?}", $valid_str, err);
                    });
                    assert_eq!(result, $expected);
                )*
            }

            #[test]
            fn test_invalid_from_str() {
                $(
                    // Avoid tests where the invalid-ness is that the code is valid for a different
                    // status type, when testing the overall `Status` enum.
                    if !$wrapped || !matches!($expected_err, Error::WrongCodeForStatus(_)) {
                        let error = match $invalid_str.parse::<$name>() {
                            Ok(result) => panic!("expected \"{}\" to not parse, parsed as {:?}", $invalid_str, result),
                            Err(err) => err,
                        };
                        assert_eq!(error, $expected_err);
                    }
                )*
            }

            #[test]
            fn test_display() {
                $(
                    let expected = $valid_str.strip_suffix("\r\n").unwrap_or($valid_str).to_string();
                    let actual = $expected.to_string();

                    // Display does not currently add newline
                    let expected = if $wrapped {
                        format!("{}\r\n", expected)
                    } else {
                        expected
                    };

                    assert_eq!(expected, actual);
                )*
            }

            #[test]
            fn test_parse_display_string() {
                $(
                    let status = $expected;
                    assert_eq!(
                        status,
                        status.to_string().parse::<$name>().unwrap(),
                        "expected an item to_string and back to be the same item"
                    );
                )*
            }

            #[test]
            fn test_get_code() {
                $(
                    let status = $expected;
                    assert_eq!($code, status.code());
                )*
            }
        };
    }

    test_status_enum! {
        module: input,
        name: Input,
        valid: {
            "10 Please enter some text" => (Code::INPUT, Input::Normal { prompt: String::from("Please enter some text") }),
            "11 Please enter some sensitive text" => (Code::SENSITIVE_INPUT, Input::Sensitive { prompt: String::from("Please enter some sensitive text") }),
            "10 This has the line ending\r\n" => (Code::INPUT, Input::Normal { prompt: String::from("This has the line ending") }),
            "11  Extra spaces are allowed" => (Code::SENSITIVE_INPUT, Input::Sensitive { prompt: String::from(" Extra spaces are allowed") })
        },
        invalid: {
            " 10 Leading spaces is bad" => Error::InvalidCode(String::new()),
            "12 Invalid 1x status" => Error::UnknownCode(12),
            "20 Success is not an input status" => Error::WrongCodeForStatus(20)
        }
    }
}
