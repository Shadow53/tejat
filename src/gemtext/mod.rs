use crate::uri::URI;

pub enum Heading<'s, S: AsRef<str> + 's = String > {
    H1(S),
    H2(S),
    H3(S),
}

pub enum RawLineType<'s, S: AsRef<str> + 's = String> {
    Blockquote(S),
    Heading(Heading<'s, S>),
    Link { target: URI, text: S },
    ListItem(S),
    Preformatted { alt_text: S, contents: S },
    Text(S),
}
