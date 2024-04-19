#![allow(dead_code, unused_variables)]

use std::{fs, hash::{DefaultHasher, Hash, Hasher}};

use error::Result;
use scraper::{ImageBytesCollection, ImageUrlCollection};
use url::Url;
use reqwest_impersonate as reqwest;
use reqwest::impersonate::Impersonate;

use crate::scraper::{nude_bird::NudeBirdScraperBuilder, ImageScraper};

mod error;
mod scraper;

fn main() -> Result<()> {
    let url = Url::parse("https://nudebird.biz/cosplay-ninja-azhaizhai-atonement-nun/")?;
    let scraper: Box<dyn ImageScraper> = Box::new(NudeBirdScraperBuilder::default()
        .url(url)
        .build()?);

    println!("Scraping image urls...");
    let urls = scraper.scrape()?;
    println!("Downloading images...");
    let images = download_images(urls)?;
    println!("Saving images...");
    save_images(images)?;
    
    Ok(())
}

pub fn download_images(image_collection: ImageUrlCollection) -> Result<ImageBytesCollection> {
    let mut images = Vec::new();
    let client = reqwest::blocking::Client::builder()
        .impersonate(Impersonate::Chrome120)
        .danger_accept_invalid_certs(true)
        .enable_ech_grease(true)
        .permute_extensions(true)
        .build()?;
    
    for url in image_collection.image_urls {
        images.push(client.get(url).send()?.bytes()?);
    }
    
    Ok(ImageBytesCollection::new(image_collection.name, image_collection.domain, images))
}

pub fn save_images(image_collection: ImageBytesCollection) -> Result<()> {
    let dir_path = format!("output/{}/{}", image_collection.domain, image_collection.name);
    fs::create_dir_all(&dir_path)?;
   
    for image in image_collection.images {
        let filepath = format!("{}/{}.jpg", &dir_path, calculate_hash(&image));
        fs::write(filepath, image)?;
    }

    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
