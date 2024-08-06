use crate::traits::{Driver, ParseWith};
use std::error::Error;
use std::path::Path;
use crate::parse::DownloadMode;

pub enum Handler {
    Shuba(Shuba),
}

pub struct Shuba;

impl std::convert::TryFrom<&str> for Handler {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("https://69shuba.cx") {
            return Ok(Handler::Shuba(Shuba));
        }

        Err("未找到与域名对应的下载器")
    }
}

impl crate::traits::Download for Shuba {
    async fn download_chapter(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<crate::traits::Driver>, Box<dyn Error>> {
        let link = url.as_ref();
        driver.goto(link).await.ok();
        let chapter = crate::model::Chapters::parse_with(&driver)
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
        let directory = crate::model::Directory::parse_with(&driver).await.unwrap();
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

            // println!("正在下载章节: {}", title);
            pb.set_message(title);
            let chapter = crate::model::Chapters::parse_with(&driver)
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

impl Handler {
    pub async fn run(
        &self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode
    ) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Box::new(crate::parse::get_driver(address, proxy_str).await?);
        let driver = match mode {
            DownloadMode::Chapter(ref link) => {
                driver.goto(link).await.ok();
                let chapter = crate::model::Chapters::parse_with(&driver)
                    .await
                    .unwrap()
                    .unwrap();
                let mut f = std::fs::File::create(
                    download_path.join(format!("{}.txt", chapter.chapters_name)),
                )?;
                use std::io::Write;
                println!("开始下载");
                println!("正在下载: {}", chapter.chapters_name);

                f.write_all(chapter.to_string().as_bytes())?;
                driver
            }
            DownloadMode::Directory(ref link) => {
                let sty = indicatif::ProgressStyle::with_template("{msg} {wide_bar} {pos}/{len} ")
                    .unwrap()
                    .progress_chars("##-");
                driver.goto(link).await.ok();
                println!("开始解析");
                let directory = crate::model::Directory::parse_with(&driver).await.unwrap();
                println!("解析完成，需要下载{}章", directory.inner_data.len());

                let pb = indicatif::ProgressBar::new(directory.inner_data.len() as u64);
                pb.set_style(sty);
                pb.set_message("开始下载");
                let mut f = std::fs::File::create(
                    download_path.join(format!("{}.txt", directory.book_name)),
                )?;

                for chapters_link in directory.inner_data {
                    let title = chapters_link.title;
                    let href = chapters_link.href;
                    driver.goto(href.as_str()).await.ok();
                    use std::io::Write;

                    // println!("正在下载章节: {}", title);
                    pb.set_message(title);
                    let chapter = crate::model::Chapters::parse_with(&driver)
                        .await
                        .unwrap()
                        .unwrap();
                    f.write_all(chapter.to_string().as_bytes())?;
                    drop(chapter);
                    pb.inc(1);
                }
                pb.finish_with_message("dene");
                driver
            }
        };
        driver.close().await.ok();
        Ok(())
    }
}
