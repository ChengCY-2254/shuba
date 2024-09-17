use crate::model::{Chapter, ChapterLink, Directory};
use crate::prelude::*;
use log::info;

pub async fn parse_dir(driver: &Driver) -> Result<Directory> {
    //这是一个button，用于展示更多章节，先按出来才能解析
    //todo 按钮要加盐
    let b = driver
        .find(By::XPath(
            "/html/body/main/div[2]/div[2]/div[2]/div[2]/div[3]/div/div[4]",
        ))
        .await;

    if let Ok(b) = b {
        let script = format!(
            "document.getElementsByClassName(\"{}\")[0].click();",
            b.attr("class")
                .await?
                .unwrap()
                .split(' ')
                .rev()
                .last()
                .unwrap()
        );
        info!("script {}", script);
        driver.execute(script.as_str(), vec![]).await.unwrap();
        // b.click().await?;
    };
    let chapters = driver
        .find_all(By::XPath(
            "/html/body/main/div[2]/div[2]/div[2]/div[2]/div[3]/div/div[3]/div",
        ))
        .await?;
    let book_name = driver
        .find(By::XPath(
            "/html/body/main/div[2]/div[1]/div[2]/div[1]/div[1]/div[2]/div[1]/span[2]",
        ))
        .await?
        .text()
        .await?;

    let url = driver.current_url().await?.to_string();
    let mut inner_data = vec![];
    // let chapter_name_selector = By::XPath("/div[1]/div[2]");
    for (i, item) in chapters.iter().enumerate() {
        // let chapter_name = item.find(chapter_name_selector).await?.text().await?;
        let chapter_name = item.text().await?;
        let json = item.attr("data-za-extra-module").await?;
        if let Some(json) = json {
            let json = serde_json::from_str::<serde_json::Value>(&json)?;
            let id = json
                .get("card")
                .unwrap()
                .get("content")
                .unwrap()
                .get("id")
                .unwrap()
                .as_str()
                .unwrap();
            // 文章
            //https://www.zhihu.com/market/paid_column/1558815650394587136/section/1558816444384784384
            //目录
            //https://www.zhihu.com/xen/market/remix/paid_column/1558815650394587136
            let dir_id = url.split('/').last().unwrap();
            let href_url = format!(
                "https://www.zhihu.com/market/paid_column/{}/section/{}",
                dir_id, id
            );
            inner_data.push(ChapterLink {
                href: href_url,
                title: chapter_name,
                id: i,
            })
        } else {
            return Err(crate::Error::ParseElement(
                "未找到json对象，请检查规则是否正常".into(),
            ))?;
        }
    }

    Ok(Directory {
        book_name,
        inner_data,
    })
}

pub async fn parse_chapter(driver: &Driver) -> Result<Chapter> {
    let title = driver.find(By::XPath("/html/body/main/div/h1")).await?;
    //todo 字符串要重新映射
    let content = driver
        .find(By::XPath("/html/body/main/div/div[2]/div[2]"))
        .await?;
    let mut builder = Chapter::builder();
    builder.chapter_name(title.text().await?);
    builder.chapter_content(content.text().await?);

    builder
        .build()
        .map_err(|e| crate::Error::ParseElement(format!("解析chapter对象失败{}", e)))
        .map_err(anyhow::Error::new)
}
