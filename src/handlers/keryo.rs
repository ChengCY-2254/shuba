#![cfg(feature = "keryo")]

use std::error::Error;
use crate::impls::keryo::{parse_with_keryo_chapter, parse_with_keryo_dir};
use crate::model::{Chapter, Directory};
use crate::traits::{BookParse, Download, Driver};

pub struct Keryo;

impl Download for Keryo {
    fn website_tips() -> Option<String> {
        Some("第二书包网获取的文本乱码居多".into())
    }
}

impl BookParse for Keryo {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter, Box<dyn Error>> {
        parse_with_keryo_chapter(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory, Box<dyn Error>> {
        parse_with_keryo_dir(driver).await
    }
}

impl crate::traits::Run for Keryo {}
