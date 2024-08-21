use tokio::join;

use crate::model::{Chapter, ChapterLink, Directory};
use crate::prelude::*;

pub async fn get_chapter_with_shuba(driver: &'_ Driver) -> Result<Chapter> {
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
            builder.chapter_content(chapters_content);
        }
        {
            let title = title.text().await?;
            builder.chapter_name(title);
        }
    }
    Ok(builder.build().unwrap())
}

/// https://github.com/ChengCY-2254/shuba/issues/5
async fn get_text_from_div(div: &fantoccini::elements::Element) -> Result<String> {
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

pub async fn get_dir_with_shuba(driver: &'_ Driver) -> Result<Directory> {
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

async fn parse_li(li: fantoccini::elements::Element) -> Result<ChapterLink> {
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
