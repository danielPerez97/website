use crate::dated_entry::{DatedEntry, IntoDatedEntryCollection};
use crate::front_matter::{FrontMatter, SplitFrontMatterAndMarkdown};
use crate::site::{BlogPost, Resume, Site};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use pulldown_cmark::{Options, Parser};
use pulldown_cmark::html::push_html;
use crate::highlight_code_blocks::HighlightCodeBlocks;
use crate::time_utils::parse_site_datetime;

pub struct SiteParser {
    root_dir: PathBuf,
    skip_syntax_highlighting: bool,
}

impl SiteParser {

    pub fn new(root_dir: &Path, skip_syntax_highlighting: bool) -> SiteParser {
        SiteParser {
            root_dir: PathBuf::from(root_dir),
            skip_syntax_highlighting,
        }
    }
    pub fn parse(&self) -> Site {
        let posts_dir = self.root_dir.join("posts");
        let mut post_entries: Vec<BlogPost> = read_dir(posts_dir)
            .ok().unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path().into_dated_entry(true))
            .map(|dated_entry| Self::parse_blog_post(self, dated_entry))
            .collect();

        post_entries.sort_by_key(|post| std::cmp::Reverse(
            parse_site_datetime(&post.date).expect("Invalid post date")
        ));

        println!("Parsed {} posts: {:?}", post_entries.len(), post_entries.iter().map(|p| &p.slug).collect::<Vec<_>>());

        let files_folder = self.root_dir.join("static/files");
        let mut resume_file: Option<PathBuf> = None;
        for entry in read_dir(files_folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            println!("{}",path.display());

            let file_name = path.file_name().unwrap().to_str().unwrap();
            if path.is_file() && file_name.ends_with("Resume.pdf") {
                resume_file = Some(path);
            }
        }

        let resume_dated_entry = resume_file.unwrap().into_dated_entry(false);
        let resume = Resume {
            last_updated: resume_dated_entry.date,
        };


        Site::from(
            post_entries,
            resume,
        )
    }

    fn parse_blog_post(&self, entry: DatedEntry) -> BlogPost {
        let blog_post = entry.content.unwrap();
        let (raw_front_matter, raw_markdown) = blog_post
            .split_into_front_matter_and_markdown();

        let entry_display = &entry.path.display();
        let raw_front_matter = raw_front_matter.unwrap();
        let front_matter: FrontMatter = yaml_serde::from_str(raw_front_matter)
            .unwrap_or_else(|_| panic!("Missing front matter: {entry_display}"));

        let title = front_matter.title;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);

        let md_parser = Parser::new_ext(raw_markdown, options);
        let mut html_output = String::new();
        let html_output = if self.skip_syntax_highlighting {
            push_html(&mut html_output, md_parser);
            html_output
        } else {
            let events = HighlightCodeBlocks::new(md_parser);
            push_html(&mut html_output, events);
            html_output
        };

        BlogPost::new(
            entry.path,
            entry.date,
            &entry.slug,
            title,
            format!("/{}/", entry.slug),
            html_output,
            front_matter.lead,
        )
    }
}