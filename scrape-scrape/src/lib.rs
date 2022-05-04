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

#[cfg(test)]
mod tests {

    use tokio;

    use crate::{UrlData, get_raw_data};
    use crate::data::*;

    #[tokio::test]
    async fn get_youtube_data() {
        let raw_data = get_raw_data("https://youtu.be/bN2iHlZTGgQ").await.unwrap();

        let data = YoutubeData {
            raw_data
        };

        let url_data = UrlData::from(data);

        assert_eq!("EastNewSound \"死生信艶、暴謳ノ絶\"Vo nayuta【東方アレンジMV】", url_data.title.unwrap());
        assert_eq!("【死生信艶、暴謳ノ絶】（ししょうしんえん、ぼうおうのぜつ）東方アレンジアルバム「Overkill Fury」Track.3Original:ZUN Vocal : nayuta　https://twitter.com/7utautaArrange : 黒鳥　https://twitter.com/ENS_Koku...", url_data.content.unwrap());
        assert_eq!("https://i.ytimg.com/vi/bN2iHlZTGgQ/maxresdefault.jpg", url_data.thumbnail.unwrap());
    }

    #[tokio::test]
    async fn get_wikipedia_data() {
        let raw_data = get_raw_data("https://en.wikipedia.org/wiki/Touhou_Project").await.unwrap();

        let data = WikipediaData {
            raw_data
        };

        let url_data = UrlData::from(data);

        assert_eq!("Touhou Project - Wikipedia", url_data.title.unwrap());
        assert_eq!(None, url_data.content);
        assert_eq!(None, url_data.thumbnail);
    }

    #[tokio::test]
    async fn get_other_website_data() {
        let raw_data = get_raw_data("https://touhou.fandom.com/wiki/Kanako_Yasaka").await.unwrap();

        let data = OtherData {
            raw_data
        };

        let url_data = UrlData::from(data);

        assert_eq!("Kanako Yasaka | Touhou Wiki | Fandom", url_data.title.unwrap());
        assert_eq!("Kanako Yasaka is the final boss on Mountain of Faith. She is Moriya Shrine's official goddess because she defeated Suwako Moriya in the Great Suwa War. Even though she was victorious, she still let Suwako hang around the shrine because she considered her a \"fellow\" goddess. During the events of Mountain of Faith, Kanako ordered Sanae to shut down the Hakurei Shrine to make Moriya Shrine the dominant shrine, but failed because of the strength of Reimu and Marisa. Kanako gave Utsuho Reiuji nuclear", url_data.content.unwrap());
        assert_eq!(None, url_data.thumbnail);
    }
}
