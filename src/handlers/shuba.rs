#![cfg(feature = "shuba")]

use std::error::Error;

use crate::model::{Chapter, Directory};
use crate::traits::{Download, Driver, Run};

pub struct Shuba;

impl Download for Shuba {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter, Box<dyn Error>> {
        Chapter::parse_with_shuba(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory, Box<dyn Error>> {
        Directory::parse_with_shuba(driver).await
    }

    fn website_tips() -> Option<String> {
        Some("69书吧需要链接到外网，请确保你的网络可以访问69shuba.cx".into())
    }
}

impl Run for Shuba {}
