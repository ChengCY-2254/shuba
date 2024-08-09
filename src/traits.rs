#![allow(dead_code)]
use crate::parse::DownloadMode;
use log::info;
use std::path::Path;

pub type Driver = fantoccini::Client;
pub type By<'a> = fantoccini::Locator<'a>;

/// 通过该trait下载内容
/// 完成后将Driver返回以供下一次调用
pub trait Download {
    /// 下载指定章节
    async fn download_chapter(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<crate::traits::Driver>, Box<dyn std::error::Error>>;
    ///下载指定目录
    async fn download_directory(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
        //下载速率，是否需要间隔多少秒
        speed: Option<f32>,
    ) -> Result<Box<crate::traits::Driver>, Box<dyn std::error::Error>>;
}
/// 每个新的解析器都需要实现这里的trait以供下载器调用
pub trait Run: Download {
    async fn run(
        &self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<String>,
        mode: crate::parse::DownloadMode,
        speed: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Box::new(crate::parse::get_driver(address, proxy_str).await?);
        driver.set_window_size(1109, 797).await.ok();
        let driver = match mode {
            DownloadMode::Chapter { url: link } => {
                info!("下载章节:{}", link);
                self.download_chapter(driver, link, download_path).await?
            }
            DownloadMode::Directory { url: link } => {
                info!("下载全本:{}", link);
                self.download_directory(driver, link, download_path, speed)
                    .await?
            }
        };
        info!("关闭浏览器");
        driver.close().await.ok();
        Ok(())
    }
}
