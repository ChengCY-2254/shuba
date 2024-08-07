#![allow(dead_code)]
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
        sleed: Option<f32>,
    ) -> Result<Box<crate::traits::Driver>, Box<dyn std::error::Error>>;
}
/// 每个新的解析器都需要实现这里的trait以供下载器调用
pub trait Run {
    async fn run(
        &self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode,
        sleed: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
