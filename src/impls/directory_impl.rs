use crate::model::{ChapterLink, Directory};
use async_trait::async_trait;
use std::error::Error;
use thirtyfour::{By, WebDriver};
use thirtyfour::prelude::ElementQueryable;

#[async_trait]
impl crate::traits::ParseWith for Directory {
    type Output = Self;

    async fn parse_with(driver: &'_ WebDriver) -> Result<Self::Output, Box<dyn Error + Send>> {
        // driver.query()
        let div = driver.query(By::Id("catalog")).first().await;
        let ul = div.unwrap().find_all(By::Tag("ul > li")).await.unwrap();
        let book_name = driver
            .find(By::XPath("/html/body/div[3]/div/h3/div[1]/a[3]"))
            .await
            .unwrap();
        let book_name = book_name.text().await.unwrap_or_default();
        let mut inner_data = vec![];
        for li in ul {
            let id: usize = li
                .attr("data-num")
                .await
                .unwrap()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
            let a = li.find(By::Tag("a")).await.unwrap();
            let href = a.attr("href").await.unwrap().unwrap_or_default();
            let title = a.text().await.unwrap_or_default();
            inner_data.push(ChapterLink { id, href, title })
        }
        
        inner_data.sort();
        
        Ok(Directory {
            book_name,
            inner_data,
        })
    }
}
