use std::error::Error;


use crate::model::{ChapterLink, Directory};
use crate::traits::{By, Driver};

impl crate::traits::ParseWith for Directory {
    type Output = Self;
    type Error = Box<dyn std::error::Error>;

    async fn parse_with(driver: &'_ Driver) -> Result<Self::Output, Box<dyn Error>> {
        let ul = driver
            .find_all(By::XPath("/html/body/div[3]/div/div[2]/ul/li"))
            .await
            .map_err(|e| format!("Failed to find li element : {}", e))?;
        let book_name = driver
            .find(By::XPath("/html/body/div[3]/div/h3/div[1]/a[3]"))
            .await
            .map_err(|e| format!("Failed to find book name : {}", e))?;
        let book_name = book_name.text().await.unwrap_or_default();
        let mut inner_data = {
            let mut data = vec![];
            for li in ul {
                let li_link = Self::parse_li(li)
                    .await
                    .map_err(|e| format!("Failed to parse li: {}", e))?;
                data.push(li_link);
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
    async fn parse_li(li: fantoccini::elements::Element) -> Result<ChapterLink, Box<dyn Error>> {
        let id: usize = li
            .attr("data-num")
            .await?
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let a = li.find(By::Css("a")).await?;
        let href = a.attr("href").await?.unwrap_or_default();
        let title = a.text().await.unwrap_or_default();
        Ok(ChapterLink { id, href, title })
    }
}
