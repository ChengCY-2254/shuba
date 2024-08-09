#![cfg(feature = "shuba")]

use std::error::Error;
use std::path::Path;

use crate::traits::{Download, Driver, Run};
use log::info;

pub struct Shuba;

impl Download for Shuba {
    async fn download_chapter(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<Driver>, Box<dyn Error>> {
        let link = url.as_ref();
        driver.goto(link).await.ok();
        let mut chapter = crate::model::Chapter::parse_with_shuba(&driver)
            .await
            .unwrap()
            .unwrap();
        info!("开始下载:{}", link);
        println!("开始下载");
        println!("正在下载: {}", chapter.chapters_name);
        // 清除版权信息
        chapter.chapters_content = chapter
            .chapters_content
            .replace("Copyright 2024 69shuba.cx", "");

        let file_name = format!("{}.txt", chapter.chapters_name);
        info!("创建文件:{}", file_name);
        let mut f = std::fs::File::create(path.join(file_name))?;
        crate::utils::format::write_chapter_by_txt(chapter, &mut f)?;

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
        let mut progress =
            crate::utils::Progress::new("{msg} {wide_bar} {pos}/{len} ", Some("##-")).unwrap();
        driver.goto(link).await.ok();
        println!("开始解析");
        println!("69书吧需要链接到外网，请确保你的网络可以访问69shuba.cx");
        let directory = crate::model::Directory::parse_with_shuba(&driver)
            .await
            .unwrap();
        println!("解析完成，需要下载{}章", directory.inner_data.len());
        let speed = if let Some(speed) = speed {
            let duration = std::time::Duration::from_millis((speed * 1000.) as u64);
            println!("每章需要等待{}s", duration.as_secs_f32());
            Some(duration)
        } else {
            None
        };
        progress.start(directory.inner_data.len() as u64);
        progress.set_message("开始下载");
        let mut f = std::fs::File::create(path.join(format!("{}.txt", directory.book_name)))?;

        for chapters_link in directory.inner_data {
            let title = chapters_link.title;
            let href = chapters_link.href;
            info!("前往地址 {}", href);
            driver.goto(href.as_str()).await.ok();
            use std::io::Write;
            progress.set_message(title.clone());
            info!("下载章节:{}", title);
            let chapter = crate::model::Chapter::parse_with_shuba(&driver)
                .await
                .unwrap()
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

impl Run for Shuba {}
