pub mod hot_girl;
pub mod nude_bird;

use bytes::Bytes;
use url::Url;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ImageUrlCollection {
    pub name: String,
    pub domain: String,
    pub image_urls: Vec<Url>,
}

#[derive(Debug, Clone)]
pub struct ImageBytesCollection {
    pub name: String,
    pub domain: String,
    pub images: Vec<Bytes>,
}

impl ImageUrlCollection {
    pub fn new(name: String, domain: String, image_urls: Vec<Url>) -> Self {
        Self { name, domain, image_urls }
    }
}

impl ImageBytesCollection {
    pub fn new(name: String, domain: String, images: Vec<Bytes>) -> Self {
        Self { name, domain, images }
    }
}

pub trait ImageScraper {
    fn scrape(&self) -> Result<ImageUrlCollection>;
}
