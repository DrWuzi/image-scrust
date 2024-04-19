use select::{document::Document, predicate::{Class, Name}};
use url::Url;

use crate::error::{Error, Result};

use derive_builder::Builder;

#[derive(Debug, Clone)]
pub struct ImageCollection {
    pub name: String,
    pub image_urls: Vec<Url>,
}

impl ImageCollection {
    pub fn new(name: String, image_urls: Vec<Url>) -> Self {
        Self { name, image_urls }
    }
}

pub trait ImageScraper {
    fn scrape(&self) -> Result<ImageCollection>;
}

#[derive(Builder, Clone)]
#[builder(build_fn(error = "Error"))]
pub struct WebsiteAScraper {
    pub url: Url,
    #[builder(default = "None")]
    pub collection_name: Option<String>,
}

impl ImageScraper for WebsiteAScraper {
    fn scrape(&self) -> Result<ImageCollection> {
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

        let response = reqwest::blocking::get(self.url.clone())?;
        let html = response.text()?;
        let mut image_urls: Vec<Url> = Vec::new();
        let pagination_selector = "pagination";
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
                
        Ok(ImageCollection::new(collection_name, image_urls))
    }
}
