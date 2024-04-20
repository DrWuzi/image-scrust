pub mod hot_girl;
pub mod nude_bird;
pub mod cosplay_tele;

use async_trait::async_trait;
use bytes::Bytes;
use url::Url;
use derive_builder::Builder;

use crate::error::{Result, Error};

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "Error"))]
pub struct ImageCollection<T> {
    pub name: String,
    pub domain: String,
    pub images: Vec<T>,
}

#[derive(Debug, Clone, Builder)]
pub struct ScrapedData {
    pub file_type: String,
    pub data: Bytes,
}

impl ScrapedData {
    pub fn new(file_type: String, data: Bytes) -> Self {
        Self { file_type, data }
    }
}

#[async_trait]
pub trait ImageScraper {
    async fn scrape(&self) -> Result<ImageCollection<Url>>;
}
