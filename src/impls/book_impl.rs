use std::error::Error;
use async_trait::async_trait;
use thirtyfour::{By, WebDriver};
use crate::model::Book;

#[async_trait]
impl crate::traits::ParseWith for Book {
    type Output = Book;

    async fn parse_with(driver: &'_ WebDriver) -> Result<Self::Output, Box<dyn Error + Send>> {
        let mut res = crate::model::BookBuilder::default();
        let meta_elements = driver.find_all(By::Tag("head > meta")).await.unwrap();
        let words_count = driver.find(By::XPath("/html/body/div[3]/div/div/ul/li[2]")).await.unwrap();

        for meta in meta_elements.iter() {
            match meta.attr("content").await.unwrap().unwrap_or_default().as_str(){
                "og:novel:book_name"=> { res.book_name(meta.attr("content").await.unwrap().unwrap()); }
                "og:novel:author"=> { res.author(meta.attr("content").await.unwrap().unwrap()); }
                "og:novel:update_time"=> { res.update_time(meta.attr("content").await.unwrap().unwrap()); }
                "og:novel:latest_chapter_name"=> { res.latest_chapter_link(meta.attr("content").await.unwrap().unwrap()); }
                _=>continue
            }
        }
        let words_count = words_count.text().await;
        res.chapters_len(words_count.unwrap_or_default().parse().unwrap_or_default());


        Ok(res.build().unwrap())
    }
}
