use std::string::ParseError;

use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};

use crate::UrlData;

pub trait Scrapable {
    fn get_title(&self, raw_data: String) -> String;

    fn get_content(&self, raw_data: &String) -> String;

}

#[derive(Serialize, Deserialize)]
pub struct YoutubeData {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct WikipediaData {
    pub url: String,
}

impl Scrapable for YoutubeData {
    fn get_title(&self, raw_data: String) -> String {
        let fragment = Html::parse_fragment(&raw_data);
        let selector = Selector::parse("meta").unwrap();

        todo!()
    }

    fn get_content(&self, raw_data: &String) -> String {
        todo!()
    }
}

impl Scrapable for WikipediaData {
    fn get_title(&self, raw_data: String) -> String {
        todo!()
    }

    fn get_content(&self, raw_data: &String) -> String {
        todo!()
    }
}

pub fn scrape<T: Scrapable>(data: T, url_data: UrlData) -> T {
    todo!()
}