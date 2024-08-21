#![cfg(feature = "zhihu")]
use crate::model::{Chapter, Directory};
use crate::prelude::*;
use crate::run_impl;
use crate::traits::BookParse;

pub struct Zhihu;

impl BookParse for Zhihu {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter> {
        crate::impls::zhihu::parse_chapter(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory> {
        crate::impls::zhihu::parse_dir(driver).await
    }
}

impl crate::traits::Download for Zhihu {
    fn website_tips() -> Option<String> {
        Some("知乎的速率开慢一点".to_string())
    }
}

run_impl!(Zhihu);
