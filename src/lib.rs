//pub mod gemtext;
pub use mime;
pub mod status;
pub mod uri;

pub fn mime_is_gemtext(m: &mime::Mime) -> bool {
    m.essence_str().eq_ignore_ascii_case("text/gemini")
}

pub fn mime_charset(m: &mime::Mime) -> mime::Name {
    m.get_param(mime::CHARSET).unwrap_or(mime::UTF_8)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
