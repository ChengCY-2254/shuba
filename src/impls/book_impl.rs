use std::error::Error;

use crate::model::Book;
use crate::traits::{Driver,By};

impl crate::traits::ParseWith for Book {
    type Output = Book;
    type Error = Box<dyn std::error::Error>;

    async fn parse_with(driver: &'_ Driver) -> Result<Self::Output, Box<dyn Error>> {
        let mut res = crate::model::BookBuilder::default();
        let meta_elements = driver.find_all(By::Css("head > meta")).await.map_err(|e|format!("Failed find meta elements : {}",e))?;
        let words_count = driver.find(By::XPath("/html/body/div[3]/div/div/ul/li[2]")).await.map_err(|e|format!("Failed find words count : {}",e))?;

        for meta in meta_elements.iter() {
            match meta.attr("content").await?.unwrap_or_default().as_str(){
                "og:novel:book_name"=> { res.book_name(meta.attr("content").await?.unwrap()); }
                "og:novel:author"=> { res.author(meta.attr("content").await?.unwrap()); }
                "og:novel:update_time"=> { res.update_time(meta.attr("content").await?.unwrap()); }
                "og:novel:latest_chapter_name"=> { res.latest_chapter_link(meta.attr("content").await?.unwrap()); }
                _=>continue
            }
        }
        let words_count = words_count.text().await;
        res.chapters_len(words_count.unwrap_or_default().parse().unwrap_or_default());


        Ok(res.build()?)
    }
}
