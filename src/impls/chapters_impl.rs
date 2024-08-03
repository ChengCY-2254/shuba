use std::error::Error;
use std::fmt::Formatter;
use tokio::join;

use crate::model::Chapters;
use crate::traits::{By, Driver};

impl crate::traits::ParseWith for Chapters {
    type Output = Option<Chapters>;
    type Error = Box<dyn std::error::Error + Send>;

    async fn parse_with(driver: &'_ Driver) -> Result<Self::Output, Box<dyn Error + Send>> {
        let mut builder = crate::model::ChaptersBuilder::default();
        let script = driver.find_all(By::XPath("/html/head/script[2]"));
        // let script = driver.find_all(By::Tag("head > script"));
        let title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let chapter_title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        //https://github.com/jonhoo/fantoccini/issues/119
        let p = driver.find_all(By::Css("p"));

        if let (Ok(_script), Ok(_title), Ok(chapter_title), Ok(p)) =
            join!(script, title, chapter_title, p)
        {
            {
                let mut chapters_content: String = String::new();
                for p in p {
                    let p = p.text().await.unwrap().trim().replace("<br>", "\r\n");
                    chapters_content.push('\n');
                    chapters_content.push_str(p.as_str());
                }
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

impl std::fmt::Display for Chapters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chapters_name)?;
        write!(
            f,
            "\n{}",
            self.chapters_content
                .replace("Copyright 2024 69shuba.cx", "")
        )?;
        write!(f, "\n\n")?;
        Ok(())
    }
}
