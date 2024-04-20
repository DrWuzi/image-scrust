#![allow(dead_code, unused_variables)]

use std::{fs, hash::{DefaultHasher, Hash, Hasher}, io::{stdin, stdout, Write}};

use error::Result;
use indicatif::ProgressBar;
use reqwest::header::{HeaderMap, ACCEPT, CACHE_CONTROL, HOST};
use scraper::{ImageCollection, ImageCollectionBuilder, ScrapedData};
use url::Url;

use crate::scraper::{nude_bird::NudeBirdScraperBuilder, hot_girl::HotGirlScraperBuilder, ImageScraper};

mod error;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
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
    let urls = scraper.scrape().await?;
    println!("Downloading images...");
    let images = download_images(urls).await?;
    println!("Saving images...");
    save_images(images)?;

    Ok(())
}

pub async fn download_images(image_collection: ImageCollection<Url>) -> Result<ImageCollection<ScrapedData>> {
    let mut scraped_data = Vec::new();
    let mut headers = HeaderMap::new();
    headers.insert(HOST, image_collection.domain.parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    let client = reqwest::Client::builder()
            .use_rustls_tls()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
            .default_headers(headers)
            .build()?;

    let pb = ProgressBar::new(image_collection.images.len() as u64);
    for url in image_collection.images {
        let file_type: String = url.to_string().rsplit(".").collect::<Vec<&str>>().get(0).unwrap_or(&"idk").to_string();
        let res = client.get(url).send().await?;
        let data = res.bytes().await?;
        scraped_data.push(ScrapedData::new(file_type, data));
        pb.inc(1);
    }
    pb.finish_with_message("completed");

    let new_collection = ImageCollectionBuilder::default()
        .name(image_collection.name)
        .domain(image_collection.domain)
        .images(scraped_data)
        .build()?;
        
    Ok(new_collection)
}

pub fn save_images(image_collection: ImageCollection<ScrapedData>) -> Result<()> {
    let dir_path = format!("output/{}/{}", image_collection.domain, image_collection.name);
    fs::create_dir_all(&dir_path)?;

    let pb = ProgressBar::new(image_collection.images.len() as u64);
    for scraped_data in image_collection.images {
        let filepath = format!("{}/{}.{}", &dir_path, calculate_hash(&scraped_data.data), scraped_data.file_type);
        fs::write(filepath, scraped_data.data)?;
        pb.inc(1);
    }
    pb.finish_with_message("completed");

    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
