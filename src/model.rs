#![allow(unused)]
/*!
    这个模块主要用于存储用于解析的数据模型
 */
use proc_macro_workshop::Builder;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::io::Write;

///Chapters Content and Name
/// https://69shuba.cx/txt/9958171/90560237
#[derive(Debug, Builder, Clone)]
pub struct Chapter {
    pub chapter_name: String,
    pub chapter_content: String,
}

/// Book Info
/// https://69shuba.cx/book/9958171.htm
#[derive(Debug, Builder, PartialEq, Eq)]
#[deprecated(note = "暂时无用")]
#[cfg(feature = "unstable")]
pub struct Book {
    pub book_name: String,
    pub author: String,
    pub update_time: String,
    pub latest_chapter_link: String,
    pub chapters_len: usize,
}

#[derive(Debug, PartialEq, Eq)]
/// https://69shuba.cx/book/9958171/
pub struct Directory {
    pub book_name: String,
    ///href data
    pub inner_data: Vec<ChapterLink>,
}

/// 用于对应链接的章节名和链接到的章节内容
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChapterLink {
    pub href: String,
    pub title: String,
    pub id: usize,
}
/// 用于存放解析后的参数
#[derive(Clone)]
pub struct CliArguments {
    pub address: String,
    pub url: Option<String>,
    pub proxy_str: Option<String>,
    pub download_path: Option<String>,
    pub speed: Option<f32>,
    #[cfg(feature = "debug")]
    pub debug: bool,
    /// 是否打印受支持的网站
    pub print_support: bool,
    /// 浏览器的登录状态存储地 存cookie
    pub user_data_dir: Option<String>,
    /// 预登录网站
    pub pre_login:bool
}

impl From<clap::ArgMatches> for CliArguments {
    fn from(matches: clap::ArgMatches) -> Self {
        let address = matches.get_one::<String>("address").unwrap().clone();
        let url = matches.get_one::<String>("url").map(String::from);
        let proxy_str: Option<String> = matches.get_one::<String>("proxy_address").cloned();
        let download_path: Option<String> = matches.get_one("download_path").cloned();
        let speed: Option<f32> = matches
            .get_one("speed")
            .map(|str: &String| str.parse::<f32>().unwrap());
        #[cfg(feature = "debug")]
        let debug = matches.get_flag("debug");
        let print_support = matches.get_flag("support_web_site");
        let user_data_dir = matches.get_one::<String>("user-data-dir").map(String::from);
        let pre_login=matches.get_flag("pre-login");

        CliArguments {
            address,
            url,
            proxy_str,
            download_path,
            speed,
            #[cfg(feature = "debug")]
            debug,
            print_support,
            user_data_dir,
            pre_login
        }
    }
}

impl std::fmt::Display for Chapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.chapter_name)?;
        write!(f, "{}", self.chapter_content)?;
        write!(f, "\n\n")?;
        Ok(())
    }
}
