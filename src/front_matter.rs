use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
}

pub trait SplitFrontMatterAndMarkdown {
    fn into_front_matter_and_markdown(&self) -> (Option<& str>, & str);
}

impl SplitFrontMatterAndMarkdown for String {
    fn into_front_matter_and_markdown(&self) -> (Option<& str>, & str) {
        split_front_matter(self)
    }
}

fn split_front_matter(content: &str) -> (Option<&str>, &str) {
    if let Some(rest) = content.strip_prefix("---\n") {
        if let Some(second) = rest.find("---\n") {
            let front_matter = &rest[..second];
            let remaining = &rest[second + 4..];
            return (Some(front_matter), remaining);
        }
    }

    (None, content)
}
