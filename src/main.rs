#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::{Path, PathBuf};

use error::{Error, Result};
use scraper::{ImageBytesCollection, ImageUrlCollection};
use url::Url;

use crate::scraper::{ImageScraper, WebsiteAScraperBuilder};

mod error;
mod scraper;

fn main() -> Result<()> {
    let url = Url::parse( "https://hotgirl.asia/hanari-하나리-djawa-romantic-lines-s-ver-set-01/").map_err(|_| Error::Generic("failed to parse url".to_string()))?;
    let scraper: Box<dyn ImageScraper> = Box::new(WebsiteAScraperBuilder::default()
        .url(url)
        .build()?);

    let collection = scraper.scrape()?;
    println!("{:?}", collection);
    
    Ok(())
}

pub fn download_images(image_collection: ImageUrlCollection) -> Result<ImageBytesCollection> {
    let mut images = Vec::new();
    
    for url in image_collection.image_urls {
        images.push(reqwest::blocking::get(url)?.bytes()?);
    }
    
    Ok(ImageBytesCollection::new(image_collection.name, images))
}

pub fn save_images(image_collection: ImageBytesCollection) -> Result<()> {
    let dir_path = format!("output/{}", image_collection.name);
    fs::create_dir_all(&dir_path)?;

    let largest_index = get_largest_index(&dir_path)?;
    
    for (index, image) in image_collection.images.iter().enumerate() {
        let filepath = format!("{}/{}.jpg", &dir_path, largest_index + index);
        fs::write(filepath, image)?;
    }

    Ok(())
}

fn get_file_names(dir_path: &str) -> Result<Vec<String>> {
    let paths = fs::read_dir(dir_path)?;

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    Ok()
}
