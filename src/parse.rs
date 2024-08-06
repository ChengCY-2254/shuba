use crate::traits::{By, ParseWith};
use fantoccini::wd::{Capabilities};
use serde_json::json;


///解析代理字符串
/// export https_proxy=http://127.0.0.1:8888;export http_proxy=http://127.0.0.1:8888;export all_proxy=socks5://127.0.0.1:8889
fn parse_proxy_caps(
    caps: &mut Capabilities,
    proxy_str: Option<&str>,
) -> Result<(), &'static str> {
    if let Some(proxy_str) = proxy_str {
        //socks5代理
        let proxy_obj = if proxy_str.starts_with("socks5://") {
            let proxy_str = proxy_str.replace("socks5://","");
            json!({
                "proxyType": "manual",
                "socksProxy": proxy_str,
                "socksVersion":5
            })
        } else {
            eprintln!("不是一个有效的socks5代理字符串，请检查你的配置");
            std::process::exit(1);
        };
        caps.insert("proxy".to_string(), proxy_obj);
    }
    Ok(())
}
#[inline]
pub async fn get_driver(
    address: &str,
    proxy_str: Option<&str>,
) -> Result<fantoccini::Client, Box<dyn std::error::Error>> {
    let mut caps = Capabilities::new();
    parse_proxy_caps(&mut caps, proxy_str)?;
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(address)
        .await
        .map_err(|e| format!("连接到WebDriver出现错误，请检查参数是否正确 {}", e).into())
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
        download_path: &std::path::Path,
        check_proxy: bool,
        proxy_str: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mode = self;
        let driver = if check_proxy {
            Box::new(self.check_connected(address, proxy_str).await?)
        } else {
            Box::new(crate::parse::get_driver(address, proxy_str).await?)
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
    ///用于检查连接是否建立并返回一个driver对象，因为69shuba屏蔽了国内ip，所以需要使用代理
    pub async fn check_connected(
        &self,
        address: &str,
        proxy_str: Option<&str>,
    ) -> Result<fantoccini::Client, Box<dyn std::error::Error>> {
        let driver = crate::parse::get_driver(address, proxy_str).await?;
        println!("检查是否连接代理");
        driver.goto("https://www.google.com").await.ok();
        driver
            .find(By::XPath("/html/body/div[1]/div[3]/form/div[1]/div[1]/div[1]/div/div[2]/textarea"))
            .await
            .map_err(|e|format!("未连接到代理，请连接到代理后再试:{:#}",e))?;
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
