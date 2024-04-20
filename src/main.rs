#![allow(dead_code, unused_variables)]

use std::{fs, hash::{DefaultHasher, Hash, Hasher}, io::{stdin, stdout, Write}};

use error::Result;
use reqwest::header::{HeaderMap, ACCEPT, CACHE_CONTROL, HOST};
use scraper::{ImageBytesCollection, ImageUrlCollection};
use url::Url;

use crate::scraper::{nude_bird::NudeBirdScraperBuilder, hot_girl::HotGirlScraperBuilder, ImageScraper};

mod error;
mod scraper;

fn main() -> Result<()> {
    let mut input = String::new();
    print!("Please enter the image url: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("failed reading input");
    
    let url = Url::parse(&input)?;
    let scraper: Box<dyn ImageScraper> = match url.domain().unwrap() {
        "nudebird.biz" => Box::new(NudeBirdScraperBuilder::default().url(url).build()?),
        "hotgirl.asia" => Box::new(HotGirlScraperBuilder::default().url(url).build()?),
        _ => panic!("unsuported url")
    };

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
    let mut headers = HeaderMap::new();
    headers.insert(HOST, image_collection.domain.parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
            .default_headers(headers)
            .build()?;
    
    for url in image_collection.image_urls {
        let res = client.get(url).send()?;
        let data = res.bytes()?;
        images.push(data);
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
