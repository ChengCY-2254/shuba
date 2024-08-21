#![cfg(feature = "shuba")]

use crate::impls::shuba::{get_chapter_with_shuba, get_dir_with_shuba};
use crate::model::{Chapter, Directory};
use crate::prelude::*;
use crate::run_impl;
use crate::traits::{BookParse, Download};

pub struct Shuba;

impl Download for Shuba {
    fn website_tips() -> Option<String> {
        Some("69书吧需要链接到外网，请确保你的网络可以访问69shuba.cx".into())
    }
}

impl BookParse for Shuba {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter> {
        get_chapter_with_shuba(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory> {
        get_dir_with_shuba(driver).await
    }
}

run_impl!(Shuba);
