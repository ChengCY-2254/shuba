use crate::traits::ParseWith;
use thirtyfour::{By, Capabilities, DesiredCapabilities};

///从字符串中选择用来抓取的浏览器
#[inline]
fn select_browser(text: &str) -> Result<Capabilities, &'static str> {
    match text.to_lowercase().as_str() {
        "chrome" => Ok(DesiredCapabilities::chrome().into()),
        "chromium" => Ok(DesiredCapabilities::chromium().into()),
        "edge" => Ok(DesiredCapabilities::edge().into()),
        "firefox" => Ok(DesiredCapabilities::firefox().into()),
        "safari" => Ok(DesiredCapabilities::safari().into()),
        "opera" => Ok(DesiredCapabilities::opera().into()),
        "ie" => Ok(DesiredCapabilities::internet_explorer().into()),
        _ => Err("missing browser"),
    }
}
#[inline]
pub async fn get_driver(
    address: &str,
    browser: &str,
) -> Result<thirtyfour::WebDriver, Box<dyn std::error::Error>> {
    let capabilities = select_browser(browser)?;
    thirtyfour::WebDriver::new(address, capabilities)
        .await
        .map_err(|e| {
            format!(
                "连接WebDriver错误，请检查参数是否正确或对应的WebDriver是否已开启\r\n{}",
                e
            )
            .into()
        })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DownloadMode {
    Chapter(String),
    Directory(String),
}

impl std::convert::TryFrom<&str> for DownloadMode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let argument_len = value
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .len();

        match argument_len {
            //https://69shuba.cx/txt/9958171/90560237
            5 => Ok(DownloadMode::Chapter(
                value.replace("book", "txt").to_string(),
            )),
            //https://69shuba.cx/book/9958171/
            4 => Ok(DownloadMode::Directory(
                value.replace("txt", "book").replace(".htm", "").to_string(),
            )),
            _ => Err("parse url error, please check out your url."),
        }
    }
}

impl DownloadMode {
    pub async fn run(
        &self,
        address: &str,
        browser: &str,
        download_path: &std::path::Path,
        check_proxy: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mode = self;
        let driver = if check_proxy {
            self.check_connected(address, browser).await?
        } else {
            crate::parse::get_driver(address, browser).await?
        };
        let driver = match mode {
            DownloadMode::Chapter(link) => {
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
            DownloadMode::Directory(link) => {
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
                    driver.goto(href).await.ok();
                    use std::io::Write;

                    // println!("正在下载章节: {}", title);
                    pb.set_message(title);
                    let chapter = crate::model::Chapters::parse_with(&driver)
                        .await
                        .unwrap()
                        .unwrap();
                    f.write_all(chapter.to_string().as_bytes())?;
                    pb.inc(1);
                }
                pb.finish_with_message("dene");
                driver
            }
        };
        driver.quit().await.ok();
        Ok(())
    }
    ///用于检查连接是否建立并返回一个driver对象，因为69shuba屏蔽了国内ip，所以需要使用代理
    pub async fn check_connected(
        &self,
        address: &str,
        browser: &str,
    ) -> Result<thirtyfour::WebDriver, Box<dyn std::error::Error>> {
        let driver = crate::parse::get_driver(address, browser).await?;
        println!("检查是否连接代理");
        driver.goto("https://www.google.com").await.ok();
        driver
            .find(By::Name("q"))
            .await
            .expect("未连接到代理，请连接到代理后再试");
        println!("已连接");
        Ok(driver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        //["https:", "69shuba.cx", "txt", "9958171", "90560237"]
        let url = "https://69shuba.cx/txt/9958171/90560237";
        let result = url.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        println!("{:?}", result);
    }
    #[test]
    fn parse_chapter_mode() {
        let expected = DownloadMode::Chapter("https://69shuba.cx/txt/9958171/90560237".to_string());

        assert_eq!(
            expected,
            DownloadMode::try_from("https://69shuba.cx/book/9958171/90560237").unwrap()
        );
    }
    #[test]
    fn parse_directory_mode() {
        let expected = DownloadMode::Directory("https://69shuba.cx/book/9958171/".to_string());

        assert_eq!(
            expected,
            DownloadMode::try_from("https://69shuba.cx/txt/9958171/").unwrap()
        );
    }
}
