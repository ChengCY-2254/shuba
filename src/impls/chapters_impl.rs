use std::error::Error;
use std::fmt::Formatter;

use thirtyfour::{By, WebDriver};
use tokio::join;

use crate::model::Chapters;

impl crate::traits::ParseWith for Chapters {
    type Output = Option<Chapters>;

    async fn parse_with(driver: &'_ WebDriver) -> Result<Self::Output, Box<dyn Error + Send>> {
        let mut builder = crate::model::ChaptersBuilder::default();
        let script = driver.find_all(By::Tag("head > script"));
        let title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let chapter_title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let p = driver.find_all(By::Tag("p"));

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
                // let chapters_name = title.text().await.unwrap_or_default();
                // builder.book_name(chapters_name);
            }
            {
                let chapters_name = chapter_title.text().await.unwrap_or_default();
                builder.chapters_name(chapters_name);
            }
            {
                // if !script.is_empty() {
                //     let script = &script[1];
                //     let script = script.text().await.unwrap_or_default();
                //     println!("{script}");
                //     let start = script.find('{').unwrap();
                //     let end = script.find('}').unwrap();
                //     let json: &str = script[start + 1..end].as_ref();
                //     let (a, b) = parse_page(json);
                //     builder.prev_page(a).next_page(b);
                // } else {
                //     return Ok(None);
                // }
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
        write!(f,"\n\n")?;
        Ok(())
    }
}
// fn parse_page(data: &str) -> (Option<String>, Option<String>) {
//     let mut a = None;
//     let mut b = None;
//     data.lines().for_each(|line| {
//         let line = line.trim();
//         if line.contains("preview_page") {
//             let start = "preview_page".len() + 1;
//             let end = line.len() - 1;
//             a = Some(
//                 line[start..end]
//                     .trim_matches(|c| c == '"' || c == ' ')
//                     .to_string(),
//             );
//         }
//         if line.contains("next_page") {
//             let start = "next_page".len() + 1;
//             let end = line.len() - 1;
//             b = Some(
//                 line[start..end]
//                     .trim_matches(|c| c == '"' || c == ' ')
//                     .to_string(),
//             );
//         }
//     });
//     (a, b)
// }
