#![cfg(feature = "ddxs")]
use crate::impls::ddxs::{get_chapter, get_dirs};
use crate::model::{Chapter, Directory};
use crate::prelude::*;
use crate::run_impl;
use crate::traits::{BookParse, Download};

pub struct Ddxs;

impl BookParse for Ddxs {
    async fn parse_chapter(driver: &Driver) -> Result<Chapter> {
        get_chapter(driver).await
    }

    async fn parse_directory(driver: &Driver) -> Result<Directory> {
        get_dirs(driver).await
    }
}

impl Download for Ddxs {
    fn website_tips() -> Option<String> {
        None
    }
}

run_impl!(Ddxs);
