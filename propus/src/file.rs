use std::path::PathBuf;

pub struct FrontMatter {
    title: Option<String>,
    author: Option<String>,
    published: Option<time::OffsetDateTime>,
    updated: Option<time::OffsetDateTime>,
    draft: bool,
}

pub struct File {
    front_matter: Option<FrontMatter>,
    path: PathBuf,
    content: String,
}
