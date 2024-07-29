use crate::traits::ParseWith;
use thirtyfour::{Capabilities, DesiredCapabilities};

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
) -> Result<thirtyfour::WebDriver, &'static str> {
    let capabilities = select_browser(browser)?;
    thirtyfour::WebDriver::new(address, capabilities)
        .await
        .map_err(|_| "连接WebDriver错误，请检查参数是否正确或对应的WebDriver是否已开启")
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let driver = match self {
            DownloadMode::Chapter(link) => {
                let driver = crate::parse::get_driver(address, browser).await?;
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
                let driver = crate::parse::get_driver(address, browser).await?;
                driver.goto(link).await.ok();
                let directory = crate::model::Directory::parse_with(&driver).await.unwrap();
                println!("解析完成，需要下载{}章", directory.inner_data.len());
                println!("开始下载");
                let mut f = std::fs::File::create(
                    download_path.join(format!("{}.txt", directory.book_name)),
                )?;

                for chapters_link in directory.inner_data {
                    let title = chapters_link.title;
                    let href = chapters_link.href;
                    driver.goto(href).await.ok();
                    use std::io::Write;

                    println!("正在下载章节: {}", title);
                    let chapter = crate::model::Chapters::parse_with(&driver)
                        .await
                        .unwrap()
                        .unwrap();
                    f.write_all(chapter.to_string().as_bytes())?;
                }
                driver
            }
        };
        driver.quit().await.ok();
        Ok(())
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
