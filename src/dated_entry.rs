use std::fs::read_to_string;
use std::path::PathBuf;
use chrono::{NaiveDate};
use crate::split_around::SplitAround;

pub struct DatedEntry {
    pub path: PathBuf,
    pub date: NaiveDate,
    pub slug: String,
    pub content: Option<String>,
}

pub trait IntoDatedEntryCollection {
    fn into_dated_entry(self, read_contents: bool) -> DatedEntry;
}

impl IntoDatedEntryCollection for PathBuf {

    fn into_dated_entry(self, read_contents: bool) -> DatedEntry {
        let file_name = self.file_stem().unwrap().to_string_lossy();
        let file_name = String::from(file_name);
        let (raw_date, slug) = file_name.split_around(10);
        let content = if read_contents {
            Some(read_to_string(&self).unwrap())
        } else {
            None
        };

        DatedEntry {
            path: self,
            date: NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d").unwrap(),
            slug,
            content,
        }
    }
}
