#![cfg(feature = "keryo")]

use crate::model::Chapter;
use crate::traits::{Download, Driver};
use log::info;
use std::error::Error;
use std::path::Path;

pub struct Keryo;

impl Download for Keryo {
    async fn download_chapter(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<Driver>, Box<dyn Error>> {
        let link = url.as_ref();
        driver.goto(link).await.ok();
        let chapter = Chapter::parse_with_keryo(&driver).await.unwrap();

        info!("开始下载:{}", link);
        println!("正在下载:{}", link);

        let file_name = format!("{}.txt", chapter.chapters_name);
        info!("创建文件{file_name}");

        let mut f = std::fs::File::create(path.join(file_name)).unwrap();
        crate::utils::format::write_chapter_by_txt(chapter, &mut f)?;
        info!("下载完成:{}", link);
        Ok(driver)
    }

    async fn download_directory(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
        speed: Option<f32>,
    ) -> Result<Box<Driver>, Box<dyn Error>> {
        let link = url.as_ref();
        let mut progress = crate::utils::default_progress();
        driver.goto(link).await.ok();
        println!("开始解析");
        let dir = crate::model::Directory::parse_with_keryo(&driver)
            .await
            .unwrap();
        println!("解析完成，需要下载{}章", dir.inner_data.len());
        let speed = crate::utils::seconds_to_millis(speed)
            .inspect(|duration| println!("每章需要等待{}s", duration.as_secs_f32()));
        progress.start(dir.inner_data.len() as u64);
        progress.set_message("开始下载");
        let mut f = std::fs::File::create(path.join(format!("{}.txt", dir.book_name)))?;

        for chapters_link in dir.inner_data {
            let title = chapters_link.title;
            let href = chapters_link.href;
            info!("前往地址 {}", href);
            driver.goto(href.as_str()).await.ok();
            use std::io::Write;
            progress.set_message(title.clone());
            info!("下载章节:{}", title);
            let chapter = crate::model::Chapter::parse_with_keryo(&driver)
                .await
                .unwrap();
            let cache = chapter.to_string();
            info!("写入文件:{},长度为{}", title, cache.len());
            f.write_all(cache.as_bytes())?;
            drop(chapter);
            progress.inc(1);
            //是否等待
            if let Some(speed) = &speed {
                tokio::time::sleep(*speed).await;
            }
        }
        progress.finish_with_message("dene");
        Ok(driver)
    }
}

impl crate::traits::Run for Keryo {}
