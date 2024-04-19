use derive_builder::Builder;
use select::{document::Document, predicate::{Class, Name}};
use url::Url;

use crate::error::{Error, Result};

use super::{ImageScraper, ImageUrlCollection};

#[derive(Builder, Clone)]
#[builder(build_fn(error = "Error"))]
pub struct HotGirlScraper {
    pub url: Url,
    #[builder(default = "None")]
    pub domain_name: Option<String>,
    #[builder(default = "None")]
    pub collection_name: Option<String>,
}

impl ImageScraper for HotGirlScraper {
    fn scrape(&self) -> Result<ImageUrlCollection> {
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

        let response = reqwest::blocking::get(self.url.clone())?;
        let html = response.text()?;
        let pagination_selector = "pagination";
        let mut image_urls: Vec<Url> = Vec::new();
        let mut page_urls: Vec<Url>= Vec::new();

        page_urls.push(self.url.clone());

        let document = Document::from(html.as_str());
        for container in document.find(Class(pagination_selector)) {
            for node in container.find(Name("li")) {
                for a in node.find(Name("a")) {
                    if let Some(link) = a.attr("href") {
                        page_urls.push(Url::parse(link)?);
                    }
                }
            }
        }

        let selector = "galeria_img";

        for page_link in page_urls {
            let response = reqwest::blocking::get(page_link)?;
            let html = response.text()?;
            let mut img_links: Vec<Url> = Vec::new();
            let document = Document::from(html.as_str());
            for container in document.find(Class(selector)) {
                for node in container.find(Name("img")) {
                    if let Some(link) = node.attr("src") {
                        img_links.push(Url::parse(link)?);
                    }
                }
            }
            
            image_urls.append(&mut img_links);
        }
                
        Ok(ImageUrlCollection::new(collection_name, domain_name, image_urls))
    }
}
