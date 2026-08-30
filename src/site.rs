use std::path::PathBuf;
use chrono::NaiveDate;
use serde::Serialize;
use crate::time_utils::format_utc;

pub struct Site {
    pub blog_posts: Vec<BlogPost>
}

impl Site {
    pub fn from(posts: Vec<BlogPost>) -> Site {
        Site {
            blog_posts: posts
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
    pub content: String
}

impl BlogPost {
    pub fn new(
       path: PathBuf,
       date: NaiveDate,
       slug: String,
       title: String,
       url: String,
       content: String
    ) -> BlogPost {
        let date = format_utc(date.and_hms_opt(0, 0, 0).unwrap().and_utc());
        BlogPost {
            path,
            date,
            slug: String::from(&slug),
            title,
            url,
            content
        }
    }
}