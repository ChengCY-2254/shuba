#![cfg(feature = "keryo")]

use std::error::Error;

use crate::model::{Chapter, Directory};
use crate::traits::{Download, Driver};

pub struct Keryo;

impl Download for Keryo {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter, Box<dyn Error>> {
        Chapter::parse_with_keryo(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory, Box<dyn Error>> {
        Directory::parse_with_keryo(driver).await
    }

    fn website_tips() -> Option<String> {
        Some("第二书包网获取的文本乱码居多".into())
    }
}

impl crate::traits::Run for Keryo {}
