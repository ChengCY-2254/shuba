#![cfg(feature = "shuba")]

use crate::parse::DownloadMode;
use crate::traits::{Download, Driver, Run};
use std::error::Error;
use std::path::Path;
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
        let chapter = crate::model::Chapters::parse_with_shuba(&driver)
            .await
            .unwrap()
            .unwrap();
        let mut f = std::fs::File::create(path.join(format!("{}.txt", chapter.chapters_name)))?;
        use std::io::Write;
        println!("开始下载");
        println!("正在下载: {}", chapter.chapters_name);

        f.write_all(chapter.to_string().as_bytes())?;
        Ok(driver)
    }

    async fn download_directory(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<crate::traits::Driver>, Box<dyn Error>> {
        let link = url.as_ref();
        let sty = indicatif::ProgressStyle::with_template("{msg} {wide_bar} {pos}/{len} ")
            .unwrap()
            .progress_chars("##-");
        driver.goto(link).await.ok();
        println!("开始解析");
        let directory = crate::model::Directory::parse_with_shuba(&driver)
            .await
            .unwrap();
        println!("解析完成，需要下载{}章", directory.inner_data.len());

        let pb = indicatif::ProgressBar::new(directory.inner_data.len() as u64);
        pb.set_style(sty);
        pb.set_message("开始下载");
        let mut f = std::fs::File::create(path.join(format!("{}.txt", directory.book_name)))?;

        for chapters_link in directory.inner_data {
            let title = chapters_link.title;
            let href = chapters_link.href;
            driver.goto(href.as_str()).await.ok();
            use std::io::Write;

            pb.set_message(title);
            let chapter = crate::model::Chapters::parse_with_shuba(&driver)
                .await
                .unwrap()
                .unwrap();
            f.write_all(chapter.to_string().as_bytes())?;
            drop(chapter);
            pb.inc(1);
        }
        pb.finish_with_message("dene");
        Ok(driver)
    }
}

impl Run for Shuba {
    async fn run(
        &self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Box::new(crate::parse::get_driver(address, proxy_str).await?);
        let driver = match mode {
            DownloadMode::Chapter(ref link) => {
                self.download_chapter(driver, link, download_path).await?
            }
            DownloadMode::Directory(ref link) => {
                self.download_directory(driver, link, download_path).await?
            }
        };
        driver.close().await.ok();
        Ok(())
    }
}
