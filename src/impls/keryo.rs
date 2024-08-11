use crate::model::{Chapter, ChapterLink, Directory};
use crate::traits::{By, Driver};
use fantoccini::elements::Element;
use log::error;
use std::error::Error;

pub async fn parse_with_keryo_dir(driver: &Driver) -> Result<Directory, Box<dyn std::error::Error>> {
    //展开目录
    driver
        .find(By::XPath("/html/body/div[4]/div[5]/div"))
        .await?
        .click()
        .await?;
    let dd_list = driver
        .find_all(By::XPath("/html/body/div[4]/div[4]/dl/dd"))
        .await?;
    let book_name = driver
        .find(By::XPath("/html/body/div[4]/div[1]/div/div/div[2]/h1"))
        .await?
        .text()
        .await?;
    let mut list = vec![];
    parse_inner_data(dd_list, &mut list).await?;

    Ok(Directory {
        book_name,
        inner_data: list,
    })
}

pub async fn parse_with_keryo_chapter(driver: &Driver) -> Result<Chapter, Box<dyn std::error::Error>> {
    let content_text = driver.find(By::Id("booktxt")).await?.text().await?;
    let chapter_title = driver
        .find(By::XPath("/html/body/div[4]/div[1]/div[2]/h1"))
        .await?
        .text()
        .await?;
    //移除第一行和最后一行
    let content_text = content_text
        .lines()
        .skip(1)
        .take(content_text.lines().count() - 2)
        .collect::<Vec<&str>>()
        .join("\n");
    Ok(Chapter {
        chapter_name: chapter_title,
        chapter_content: content_text,
    })
}

/// 将元素转化成链接对
async fn parse_inner_data(
    dd_list: Vec<Element>,
    ret: &mut Vec<ChapterLink>,
) -> Result<(), Box<dyn Error>> {
    // println!("dd len {}", dd_list.len());
    for (i, dd) in dd_list.iter().enumerate() {
        let class_name = dd.attr("class").await?;
        if let Some(class_name) = class_name {
            if class_name == "col-sm-4" {
                let a = dd.find(By::Css("a")).await?;
                let href = a.attr("href").await?;
                if let Some(href) = href {
                    // 找到其中的数字名称解析并放入list中
                    let chapter_name = a.text().await?;
                    if let Some(number) = find_number(&chapter_name) {
                        let link = ChapterLink {
                            href,
                            title: chapter_name,
                            id: number,
                        };
                        ret.push(link);
                    } else {
                        // err_count += 1;
                        let link = ChapterLink {
                            href,
                            title: chapter_name.clone(),
                            id: i + 1,
                        };
                        ret.push(link);
                        error!(
                            "无法解析该章节名称的顺序:{}，请检查章节名称中是否包含了数字信息",
                            chapter_name
                        )
                    }
                } else {
                    continue;
                }
            }
        }
    }
    Ok(())
}

fn find_number(text: &str) -> Option<usize> {
    use crate::utils::text::{find_chinese_number, find_number};

    if let Some(number) = find_number(text) {
        return Some(number);
    }

    if let Some(number) = find_chinese_number(text) {
        return Some(number);
    }

    None
}
