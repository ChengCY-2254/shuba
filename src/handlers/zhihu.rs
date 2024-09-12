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
        Some("B乎接口加密👍，要对字符集重映射".to_string())
    }
}

run_impl!(Zhihu);
