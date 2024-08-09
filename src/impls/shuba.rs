use crate::model::{Book, Chapter, ChapterLink, Directory};
use crate::traits::{By, Driver};
use std::error::Error;
use tokio::join;

impl Book {
    pub async fn parse_with_shuba(driver: &'_ Driver) -> Result<Book, Box<dyn Error>> {
        let mut res = crate::model::Book::builder();
        let meta_elements = driver
            .find_all(By::Css("head > meta"))
            .await
            .map_err(|e| format!("Failed find meta elements : {}", e))?;
        let words_count = driver
            .find(By::XPath("/html/body/div[3]/div/div/ul/li[2]"))
            .await
            .map_err(|e| format!("Failed find words count : {}", e))?;

        for meta in meta_elements.iter() {
            match meta.attr("content").await?.unwrap_or_default().as_str() {
                "og:novel:book_name" => {
                    res.book_name(meta.attr("content").await?.unwrap());
                }
                "og:novel:author" => {
                    res.author(meta.attr("content").await?.unwrap());
                }
                "og:novel:update_time" => {
                    res.update_time(meta.attr("content").await?.unwrap());
                }
                "og:novel:latest_chapter_name" => {
                    res.latest_chapter_link(meta.attr("content").await?.unwrap());
                }
                _ => continue,
            }
        }
        let words_count = words_count.text().await;
        res.chapters_len(words_count.unwrap_or_default().parse().unwrap_or_default());

        res.build()
    }
}

impl Chapter {
    pub async fn parse_with_shuba(driver: &'_ Driver) -> Result<Chapter, Box<dyn Error>> {
        let mut builder = crate::model::Chapter::builder();
        let script = driver.find_all(By::XPath("/html/head/script[2]"));
        let title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let chapter_title = driver.find(By::XPath("/html/body/div[2]/div[1]/div[3]/h1"));
        let text_block = driver.find_all(By::XPath("html/body/div[2]/div[1]/div[3]"));

        if let (Ok(_script), Ok(title), Ok(_chapter_title), Ok(text_block)) =
            join!(script, title, chapter_title, text_block)
        {
            {
                let text_block = text_block.first().expect("no text block found");
                let chapters_content = get_text_from_div(text_block).await?;
                builder.chapters_content(chapters_content);
            }
            {
                let title = title.text().await?;
                builder.chapters_name(title);
            }
        }
        Ok(builder.build().unwrap())
    }
}
/// https://github.com/ChengCY-2254/shuba/issues/5
async fn get_text_from_div(div: &fantoccini::elements::Element) -> Result<String, Box<dyn Error>> {
    div.text()
        .await
        .map(|s| {
            s.replace("<br>", "\r\n")
                .lines()
                .skip(3)
                //这里使用fold是为了保持换行格式
                .fold(String::new(), |mut acc, r| {
                    acc.push_str(r);
                    acc.push('\n');
                    acc
                })
        })
        .map_err(|e| format!("get text from div error:{}", e).into())
}

impl Directory {
    pub async fn parse_with_shuba(driver: &'_ Driver) -> Result<Directory, Box<dyn Error>> {
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
                let li_link = parse_li(li)
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
