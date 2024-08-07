#![allow(unused)]

use crate::parse::Format;
use proc_macro_workshop::Builder;

///Chapters Content and Name
/// https://69shuba.cx/txt/9958171/90560237
#[derive(Debug, Builder, Clone)]
pub struct Chapters {
    pub chapters_name: String,
    pub chapters_content: String,
}

/// Book Info
/// https://69shuba.cx/book/9958171.htm
#[derive(Debug, Builder, PartialEq, Eq)]
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
pub struct CliArguments {
    pub address: String,
    pub url: String,
    pub proxy_str: Option<String>,
    pub download_path: Option<String>,
    pub speed: Option<f32>,
    pub debug: bool,
    pub format: Format,
}

impl From<clap::ArgMatches> for CliArguments {
    fn from(matches: clap::ArgMatches) -> Self {
        let address = matches.get_one::<String>("address").unwrap().clone();
        let url = matches.get_one::<String>("url").unwrap().clone();
        let proxy_str: Option<String> = matches.get_one::<String>("proxy_address").cloned();
        let download_path: Option<String> = matches.get_one("download_path").cloned();
        let speed: Option<f32> = matches
            .get_one("speed")
            .map(|str: &String| str.parse::<f32>().unwrap());
        let debug = matches.get_flag("debug");
        let format: &Format = matches
            .get_one::<Format>("download_format")
            .unwrap_or(&Format::Txt);
        
        CliArguments {
            address,
            url,
            proxy_str,
            download_path,
            speed,
            debug,
            format: format.clone(),
        }
    }
}
