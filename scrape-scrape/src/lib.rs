#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused)]

use reqwest;

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

impl UrlData {
    pub fn get_title(&self, raw_data: &String) -> String {
        todo!()
    }

    pub fn get_content(&self, raw_data: &String) -> String {
        todo!()
    }
}

pub async fn get_raw_data(url: String) -> Result<String, reqwest::Error> {
    Ok(reqwest::get(&url).await?.text().await?)
}
