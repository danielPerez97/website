use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
}

pub trait SplitFrontMatterAndMarkdown<'a> {
    fn split_into_front_matter_and_markdown(&'a self) -> (Option<&'a str>, &'a str);
}

impl<'a> SplitFrontMatterAndMarkdown <'a>  for String {
    fn split_into_front_matter_and_markdown(&'a self) -> (Option<&'a str>, &'a str) {
        let str = self;
        split_front_matter(str)
    }
}

fn split_front_matter(content: &str) -> (Option<&str>, &str) {
    if let Some(rest) = content.strip_prefix("---\n")
        && let Some(second) = rest.find("---\n")
    {
        let front_matter = &rest[..second];
        let remaining = &rest[second + 4..];
        return (Some(front_matter), remaining);
    }

    (None, content)
}
