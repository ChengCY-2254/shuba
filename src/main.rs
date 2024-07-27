use thirtyfour::{DesiredCapabilities, WebDriver};

use crate::traits::ParseWith;

mod argument_parse;
mod impls;
mod model;
mod traits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = argument_parse::ArgsParse::parse(std::env::args())?;
    let url_arguments = cli_args
        .txt_link
        .as_str()
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let downloads = std::env::current_dir()?.join("downloads");
    if !downloads.exists() {
        std::fs::create_dir_all(&downloads)?;
    }

    let driver = match url_arguments.len() {
        5 => {
            let caps = DesiredCapabilities::edge();
            let driver = WebDriver::new("http://localhost:9515", caps).await?;
            driver.goto(cli_args.txt_link).await.ok();
            let chapter = model::Chapters::parse_with(&driver).await.unwrap().unwrap();
            let mut f =
                std::fs::File::create(downloads.join(format!("{}.txt", chapter.chapters_name)))?;
            use std::io::Write;

            println!("正在下载: {}", chapter.chapters_name);
            f.write_all(chapter.chapters_name.as_bytes())?;
            f.write_all(b"\n")?;
            f.write_all(
                chapter
                    .chapters_content
                    .replace("Copyright 2024 69shuba.cx", "")
                    .as_bytes(),
            )?;
            Some(driver)
        }
        4 => {
            let caps = DesiredCapabilities::edge();
            let driver = WebDriver::new("http://localhost:9515", caps).await?;
            let url = cli_args.txt_link.replace("txt", "book");
            driver.goto(url).await.ok();
            let dirctory = model::Directory::parse_with(&driver).await.unwrap();
            println!("解析完成，需要下载{}章", dirctory.inner_data.len());
            let mut f =
                std::fs::File::create(downloads.join(format!("{}.txt", dirctory.book_name)))?;

            for chapters_link in dirctory.inner_data {
                let title = chapters_link.title;
                let href = chapters_link.href;
                driver.goto(href).await.ok();
                use std::io::Write;

                //写入书名
                f.write_all(format!("{}\n", title).as_bytes()).unwrap();
                println!("正在下载: {}", title);
                let chapter = model::Chapters::parse_with(&driver).await.unwrap().unwrap();
                f.write_all(
                    chapter
                        .chapters_content
                        .replace("Copyright 2024 69shuba.cx", "")
                        .as_bytes(),
                )?;
                f.write_all(b"\n\n")?;
            }

            Some(driver)
        }
        _ => None,
    };
    driver.unwrap().quit().await.ok();
    Ok(())
}
