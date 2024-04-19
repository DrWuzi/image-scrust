#![allow(dead_code, unused_variables)]

use error::{Error, Result};
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

