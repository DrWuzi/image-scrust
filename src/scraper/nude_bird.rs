use async_trait::async_trait;
use derive_builder::Builder;
use select::{document::Document, predicate::{Class, Name}};
use url::Url;

use crate::error::{Error, Result};

use super::{ImageScraper, ImageUrlCollection};

#[derive(Builder, Clone)]
#[builder(public, build_fn(error = "Error"))]
pub struct NudeBirdScraper {
    pub url: Url,
    #[builder(default = "None")]
    pub domain_name: Option<String>,
    #[builder(default = "None")]
    pub collection_name: Option<String>,
}

#[async_trait]
impl ImageScraper for NudeBirdScraper {
    async fn scrape(&self) -> Result<ImageUrlCollection> {
        let collection_name = match &self.collection_name {
            Some(name) => name.clone(),
            None => {
                let path_segments = self.url.path_segments();

                urlencoding::decode(match path_segments {
                    Some(mut split) => split.next().unwrap_or("default"),
                    None => "default",
                }).map_err(|_| Error::Generic("failed to decode url".to_string()))?.into_owned()
            },
        };

        let domain_name = match &self.domain_name {
            Some(name) => name.clone(),
            None => self.url.domain().unwrap_or("unknown").to_string(),
        };

        let response = reqwest::get(self.url.clone()).await?;
        let html = response.text().await?;
        let mut image_urls: Vec<Url> = Vec::new();

        let selector = "thecontent";
        let document = Document::from(html.as_str());
        for container in document.find(Class(selector)) {
            for p_node in container.find(Name("p")) {
                for a_node in p_node.find(Name("a")) {
                    if let Some(link) = a_node.attr("href") {
                        image_urls.push(Url::parse(link)?);
                    }
                }
            }
        }

        Ok(ImageUrlCollection::new(collection_name, domain_name, image_urls))
    }
}
