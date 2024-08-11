use crate::model::{Chapter, ChapterLink, Directory};
use crate::traits::{By, Driver};

pub async fn get_dirs(driver: &Driver) -> Result<Directory, Box<dyn std::error::Error>> {
    let tbody = driver
        .find_all(By::XPath(
            "/html/body/div[3]/div[2]/dl/dd[3]/table[2]/tbody",
        ))
        .await?;

    let a_list = tbody
        .first()
        .expect("未找到章节链接，请联系开发者进行反馈")
        .find_all(By::Css("a"))
        .await?;

    let book_name = driver
        .find(By::XPath(
            "/html/body/div[3]/div[2]/dl/dd[1]/div[2]/div[1]/h1",
        ))
        .await?
        .text()
        .await?;

    let mut inner_data = vec![];
    for (i, e) in a_list.iter().enumerate() {
        let chapter_name = e.text().await?;
        let href = e
            .attr("href")
            .await?
            .unwrap_or_else(|| panic!("{} href链接丢失", chapter_name));
        let id = crate::utils::text::find_number(&chapter_name).unwrap_or(i);
        inner_data.push(ChapterLink {
            href,
            title: chapter_name,
            id,
        })
    }

    Ok(Directory {
        book_name,
        inner_data,
    })
}
pub async fn get_chapter(driver: &Driver) -> Result<Chapter, Box<dyn std::error::Error>> {
    let content_dd = driver
        .find(By::Id("contents"))
        .await?;
    let h1 = driver
        .find(By::Css("#amain > dl > dd:nth-child(2) > h1"))
        .await?;
    let chapter_content = content_dd.text().await?;
    let chapter_name = h1.text().await?;

    Ok(Chapter {
        chapter_content,
        chapter_name
    })
}
