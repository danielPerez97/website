use std::path::PathBuf;
use chrono::NaiveDate;
use serde::Serialize;
use crate::time_utils::format_utc;

pub struct Site {
    pub blog_posts: Vec<BlogPost>,
    pub resume: Resume,
}

impl Site {
    pub fn from(
        posts: Vec<BlogPost>,
        resume: Resume,
    ) -> Site {
        Site {
            blog_posts: posts,
            resume
        }
    }
}

#[derive(Serialize)]
pub struct BlogPost {
    pub path: PathBuf,
    pub date: String,
    pub slug: String,
    pub title: String,
    pub url: String,
    pub content: String,
    pub lead: String,
}

impl BlogPost {
    pub fn new(
       path: PathBuf,
       date: NaiveDate,
       slug: &str,
       title: String,
       url: String,
       content: String,
       lead: String,
    ) -> BlogPost {
        let date = format_utc(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        BlogPost {
            path,
            date,
            slug: String::from(slug),
            title,
            url,
            content,
            lead
        }
    }
}

pub struct Resume {
    pub last_updated: NaiveDate,
}