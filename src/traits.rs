#![allow(dead_code)]
use std::path::Path;

pub type Driver = fantoccini::Client;
pub type By<'a> = fantoccini::Locator<'a>;

pub trait ParseWith<T = Driver> {
    type Output;
    type Error;
    async fn parse_with(_driver: &'_ T) -> Result<Self::Output, Self::Error>;
}

/// 通过该trait下载内容
/// 通过选择url来匹配主机地址，将内容返回为Box<dyn Download>让一个空结构体来实现Download trait，然后通过解析不同的主机地址返回不同的下载枚举。
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
    ) -> Result<Box<crate::traits::Driver>, Box<dyn std::error::Error>>;
    
}