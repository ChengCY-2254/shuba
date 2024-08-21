#![cfg(feature = "keryo")]

use crate::impls::keryo::{parse_with_keryo_chapter, parse_with_keryo_dir};
use crate::model::{Chapter, Directory};
use crate::prelude::*;
use crate::run_impl;
use crate::traits::{BookParse, Download};

pub struct Keryo;

impl Download for Keryo {
    fn website_tips() -> Option<String> {
        Some("第二书包网获取的文本乱码居多".into())
    }
}

impl BookParse for Keryo {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter> {
        parse_with_keryo_chapter(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory> {
        parse_with_keryo_dir(driver).await
    }
}

run_impl!(Keryo);
