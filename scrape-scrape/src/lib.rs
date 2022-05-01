#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]

use reqwest;
use scraper::{Html, Selector};

pub mod data;

pub struct UrlData {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Default for UrlData {
    fn default() -> Self {
        Self {
            title: None,
            content: None,
        }
    }
}

pub async fn get_raw_data(url: String) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(&url).await?.text().await?)
}
