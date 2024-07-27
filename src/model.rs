#![allow(unused)]
use derive_builder::Builder;

///Chapters Content and Name
/// https://69shuba.cx/txt/9958171/90560237
#[derive(Debug, Builder, Clone)]
pub struct Chapters {
    // pub book_name:String,
    pub chapters_name: String,
    pub chapters_content: String,
    // #[builder(default)]
    // pub prev_page: Option<String>,
    // #[builder(default)]
    // pub next_page: Option<String>,
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

impl Book {
    pub fn directory_link(&self) -> Result<String, &'static str> {
        self.latest_chapter_link
            .rfind('/')
            .map(|i| self.latest_chapter_link[0..i + 1].replace("txt", "book"))
            .ok_or("No directory link")
    }
}

#[derive(Debug, PartialEq, Eq)]
/// https://69shuba.cx/book/9958171/
pub struct Directory {
    pub book_name: String,
    ///href data
    pub inner_data: Vec<ChapterLink>,
}

// #catalog > ul > li
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChapterLink {
    pub href: String,
    pub title: String,
    pub id: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_link_assert() {
        let lbook = Book {
            book_name: "魔法少女餐厅".to_string(),
            author: "凤吃鱼".to_string(),
            update_time: "2022-10-18".to_string(),
            latest_chapter_link: "https://69shuba.cx/txt/9958171/90560237".to_string(),
            chapters_len: 486,
        };
        assert_eq!(
            lbook.directory_link().unwrap(),
            "https://69shuba.cx/book/9958171/"
        );
    }
}