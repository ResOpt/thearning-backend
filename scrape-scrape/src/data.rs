use std::os::unix::prelude::OsStrExt;
use std::string::ParseError;
use std::io::Read;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use select::document::Document;
use select::predicate::{Class, Name, Predicate, Attr};

use crate::UrlData;

pub trait Scrapable {
    fn get_title(&self) -> Option<String>;

    fn get_content(&self) -> Option<String>;

    fn get_thumbnail(&self) -> Option<String>; 
}

#[derive(Serialize, Deserialize)]
pub struct YoutubeData {
    pub raw_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct WikipediaData {
    pub raw_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct OtherData {
    pub raw_data: String,
}

impl Scrapable for YoutubeData {
    fn get_title(&self) -> Option<String> {

        let docs = Document::from(self.raw_data.as_str());

        docs.find(Attr("property", "og:title")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>()

    }

    fn get_content(&self) -> Option<String> {

        let docs = Document::from(self.raw_data.as_str());

        docs.find(Attr("property", "og:description")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>()

    }

    fn get_thumbnail(&self) -> Option<String> {

        let docs = Document::from(self.raw_data.as_str());

        docs.find(Attr("property", "og:image")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>()    
    }
}

impl Scrapable for WikipediaData {
    fn get_title(&self) -> Option<String> {

        let docs = Document::from(self.raw_data.as_str());

        docs.find(Attr("property", "og:title")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>()    
    
    }

    fn get_content(&self) -> Option<String> {
        None
    }

    fn get_thumbnail(&self) -> Option<String> {
        None
    }
}

impl Scrapable for OtherData {
    fn get_title(&self) -> Option<String> {
        let docs = Html::parse_document(&self.raw_data);
        let selector = Selector::parse("title").unwrap();

        let title = docs.select(&selector).next().unwrap();
        title.text().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().first().cloned()
    }

    fn get_content(&self) -> Option<String> {
        let docs = Document::from(self.raw_data.as_str());

        match docs.find(Attr("property", "og:description")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>() {
            Some(desc) => Some(desc),
            None => match docs.find(Attr("name", "description")).into_iter().map(|x| x.attr("content")).collect::<Option<String>>() {
                Some(desc) => Some(desc),
                None => None
            }
        }
    }

    fn get_thumbnail(&self) -> Option<String> {
        None
    }
}