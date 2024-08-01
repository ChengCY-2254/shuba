use crate::model::{ChapterLink, Directory};
use std::error::Error;
use thirtyfour::prelude::ElementQueryable;
use thirtyfour::{By, WebDriver, WebElement};

impl crate::traits::ParseWith for Directory {
    
    type Output = Self;

    async fn parse_with(driver: &'_ WebDriver) -> Result<Self::Output, Box<dyn Error + Send>> {
        let div = driver.query(By::Id("catalog")).first().await;
        let ul = div.unwrap().find_all(By::Tag("ul > li")).await.unwrap();
        let book_name = driver
            .find(By::XPath("/html/body/div[3]/div/h3/div[1]/a[3]"))
            .await
            .unwrap();
        let book_name = book_name.text().await.unwrap_or_default();
        let mut inner_data = {
            let mut data = vec![];
            for li in ul {
                data.push(Self::parse_li(li).await)
            }
            data
        };
        inner_data.sort();

        Ok(Directory {
            book_name,
            inner_data,
        })
    }
}

impl Directory {
    async fn parse_li(li: WebElement) -> ChapterLink {
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
        ChapterLink { id, href, title }
    }
}
