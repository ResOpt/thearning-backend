#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]

use data::Scrapable;
use reqwest;
use serde::{Serialize, Deserialize};

pub mod data;

pub enum Url {
    Youtube,
    Wikipedia,
    Other,
}

impl From<String> for Url {
    fn from(data: String) -> Self {
        match data {
            s if s.contains("youtube.com") || s.contains("youtu.be") => {
                Self::Youtube
            }
            s if s.contains("wikipedia") => {
                Self::Wikipedia
            }
            _ => {
                Self::Other
            }
        }
    }
}

impl From<&String> for Url {
    fn from(data: &String) -> Self {
        match data {
            s if s.contains("youtube.com") || s.contains("youtu.be") => {
                Self::Youtube
            }
            s if s.contains("wikipedia") => {
                Self::Wikipedia
            }
            _ => {
                Self::Other
            }
        }
    }
}

impl From<&str> for Url {
    fn from(data: &str) -> Self {
        match data {
            s if s.to_string().contains("youtube.com") || s.to_string().contains("youtu.be") => {
                Self::Youtube
            }
            s if s.to_string().contains("wikipedia") => {
                Self::Wikipedia
            }
            _ => {
                Self::Other
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UrlData {
    pub title: Option<String>,
    pub content: Option<String>,
    pub thumbnail: Option<String>,
}

impl Default for UrlData {
    fn default() -> Self {
        Self {
            title: None,
            content: None,
            thumbnail: None,
        }
    }
}

impl<T> From<T> for UrlData
where T: Scrapable {
    fn from(data: T) -> Self {
    
        let title = data.get_title();
    
        let content = data.get_content();
    
        let thumbnail = data.get_thumbnail();

        Self {
            title,
            content,
            thumbnail
        }
    }
}

pub async fn get_raw_data(url: &str) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(url).await?.text().await?)
}
