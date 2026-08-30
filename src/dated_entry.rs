use std::fs::read_to_string;
use std::path::PathBuf;
use chrono::{NaiveDate};
use crate::split_around::SplitAround;

pub struct DatedEntry {
    pub path: PathBuf,
    pub date: NaiveDate,
    pub slug: String,
    pub content: String,
}

pub trait IntoDatedEntryCollection {
    fn into_dated_entry(&self) -> DatedEntry;
}

impl IntoDatedEntryCollection for PathBuf {

    fn into_dated_entry(&self) -> DatedEntry {
        let file_name = self.file_stem().unwrap().to_string_lossy();
        let file_name = String::from(file_name);
        let (raw_date, slug) = file_name.split_around(10);
        let content = read_to_string(self).unwrap();

        DatedEntry {
            path: PathBuf::from(self),
            date: NaiveDate::parse_from_str(&raw_date, "%Y-%m-%d").unwrap(),
            slug: String::from(slug),
            content,
        }
    }
}
