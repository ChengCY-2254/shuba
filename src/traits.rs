#![allow(dead_code, unused_imports)]

use crate::model::{Chapter, Directory};
use crate::prelude::*;
use crate::router::Router;
use log::info;
use std::borrow::Cow;
use std::ops::Deref;
use std::path::Path;

/// 通过该trait下载内容
/// 完成后将Driver返回以供下一次调用
#[cfg(feature = "web-driver")]
pub trait Download: BookParse {
    /// 下载指定章节
    async fn download_chapter(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
    ) -> Result<Box<Driver>> {
        let link = url.as_ref();
        driver.goto(link).await.ok();
        let chapter = Self::parse_chapter(&driver).await.unwrap();

        info!("开始下载:{}", link);
        println!("正在下载:{}", link);

        let file_name = format!("{}.txt", chapter.chapter_name);
        info!("创建文件{file_name}");

        let mut f = std::fs::File::create(path.join(&file_name)).unwrap();
        crate::utils::format::write_chapter_by_txt(chapter, &mut f).unwrap();
        info!("{} 下载完成", file_name);
        Ok(driver)
    }
    ///下载指定目录
    async fn download_directory(
        &self,
        driver: Box<Driver>,
        url: impl AsRef<str>,
        path: &Path,
        //下载速率，是否需要间隔多少秒
        speed: Option<f32>,
    ) -> Result<Box<Driver>> {
        let link = url.as_ref();
        let mut progress = crate::utils::default_progress();
        driver.goto(link).await.ok();
        if let Some(help_msg) = Self::website_tips() {
            println!("{help_msg}")
        }
        println!("开始解析");
        let dir = Self::parse_directory(&driver).await.unwrap();
        println!("解析完成，需要下载{}章", dir.inner_data.len());
        let speed = crate::utils::seconds_to_millis(speed).inspect(|duration| {
            println!("每章需要等待{}s", duration.as_secs_f32());
            info!("每章需要等待{}s", duration.as_secs_f32())
        });
        progress.start(dir.inner_data.len() as u64);
        progress.set_message("开始下载");

        let mut f = {
            let path = path.join(format!("{}.txt", dir.book_name));
            info!("创建文件 {}", path.display());
            std::fs::File::create(path).unwrap()
        };

        for chapters_link in dir.inner_data {
            let title = chapters_link.title;
            let href = chapters_link.href;
            info!("前往地址 {}", href);
            driver.goto(href.as_str()).await.ok();
            use std::io::Write;
            progress.set_message(title.clone());
            info!("下载章节:{}", title);
            let chapter = Self::parse_chapter(&driver).await.unwrap();

            let cache = chapter.to_string();
            info!("写入文件:{},长度为{}", title, cache.len());
            f.write_all(cache.as_bytes()).unwrap();
            drop(chapter);
            progress.inc(1);
            //是否等待
            if let Some(speed) = &speed {
                tokio::time::sleep(*speed).await;
            }
        }
        info!("全本下载结束");
        progress.finish_with_message("dene");
        Ok(driver)
    }
    /// 对应下载器所提供的一些提示信息
    fn website_tips() -> Option<String>;
}
/// 每个新的解析器都需要实现这里的trait以供下载器调用
#[cfg(feature = "web-driver")]
pub trait Run: Download {
    async fn run(
        &self,
        address: &str,
        download_path: &Path,
        proxy_str: Option<String>,
        router: Router,
        speed: Option<f32>,
        user_data_file: Option<String>,
    ) -> Result<()> {
        let driver = Box::new(crate::parse::get_driver(address, proxy_str).await?);
        driver.set_window_size(1109, 797).await.ok();
        //获取user_data_dir中存储的用户数据 cookie
        let driver = match router {
            Router::Chapter { url: link } => {
                add_cookies(&user_data_file, &driver, &link).await?;
                info!("下载章节:{}", link);
                self.download_chapter(driver, link, download_path).await?
            }
            Router::Directory { url: link } => {
                add_cookies(&user_data_file, &driver, &link).await?;
                info!("下载全本:{}", link);
                self.download_directory(driver, link, download_path, speed)
                    .await?
            }
        };
        // if let Some(file) = user_data_dir {
        //     //获取全cookie
        //     let cookies = driver.get_all_cookies().await?;
        //     info!("回写cookie");
        //     crate::parse::cookie::write_cookies(file, cookies)?;
        // }
        info!("关闭浏览器");
        driver.close().await.ok();
        Ok(())
    }
}
async fn add_cookies<S: AsRef<str>>(
    user_data_file: &Option<String>,
    driver: &Driver,
    url: S,
) -> Result<()> {
    if let Some(file) = &user_data_file {
        info!("正在读取cookie");
        driver.goto(url.as_ref()).await?;

        if std::path::Path::new(file).exists() {
            let cookie_file = std::fs::read_to_string(file)?;
            let iter = cookie_file.split_inclusive('\n').map(|line| {
                let Some(line) = line.strip_suffix('\n') else {
                    return Cow::Owned(line.to_string());
                };
                let Some(line) = line.strip_suffix('\r') else {
                    return Cow::Owned(line.to_string());
                };
                Cow::Owned(line.to_string())
            });
            let cookies = crate::parse::cookie::read_cookies(iter)?;
            for cookie in cookies {
                info!("正在添加 {} 的cookie", cookie.domain().unwrap());
                driver.add_cookie(cookie).await.ok();
            }
        }
    }
    Ok(())
}

pub trait BookParse {
    /// 默认跳到了指定页面，才移交给解析器
    #[cfg(feature = "web-driver")]
    async fn parse_chapter(driver: &Driver) -> Result<Chapter>;

    /// 默认跳到了指定页面，才移交给解析器
    #[cfg(feature = "web-driver")]
    async fn parse_directory(driver: &Driver) -> Result<Directory>;
    //#[cfg(all(not(feature = "web-driver"),feature = "request"))]
}
