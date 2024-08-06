use std::error::Error;
use std::fmt::Formatter;
use tokio::join;

use crate::model::Chapters;
use crate::traits::{By, Driver};

impl crate::traits::ParseWith for Chapters {
    type Output = Option<Chapters>;
    type Error = Box<dyn std::error::Error>;

    async fn parse_with(driver: &'_ Driver) -> Result<Self::Output, Box<dyn Error>> {
        let mut builder = crate::model::ChaptersBuilder::default();
        let script = driver.find_all(By::XPath("/html/head/script[2]"));
        let title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let chapter_title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let text_block = driver.find_all(By::XPath("html/body/div[2]/div[1]/div[3]"));

        if let (Ok(_script), Ok(_title), Ok(chapter_title), Ok(text_block)) =
            join!(script, title, chapter_title, text_block)
        {
            {
                let text_block = text_block.first().expect("no text block found");
                let chapters_content = get_text_from_div(text_block).await?;
                builder.chapters_content(chapters_content);
            }
            {
                let chapters_name = chapter_title.text().await.unwrap_or_default();
                builder.chapters_name(chapters_name);
            }
        }
        Ok(Some(builder.build().unwrap()))
    }
}
/// https://github.com/ChengCY-2254/shuba/issues/5
async fn get_text_from_div(div: &fantoccini::elements::Element) -> Result<String, Box<dyn Error>> {
    div.text()
        .await
        .map(|s| {
            s.replace("<br>", "\r\n")
                .lines()
                .skip(2)
                //这里使用fold是为了保持换行格式
                .fold(String::new(), |mut acc, r| {
                    acc.push_str(r);
                    acc.push('\n');
                    acc
                })
        })
        .map_err(|e| format!("get text from div error:{}", e).into())
}

impl std::fmt::Display for Chapters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.chapters_content
                .replace("Copyright 2024 69shuba.cx", "")
        )?;
        write!(f, "\n\n")?;
        Ok(())
    }
}
