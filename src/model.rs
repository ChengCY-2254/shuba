#![allow(unused)]
use derive_builder::Builder;

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
