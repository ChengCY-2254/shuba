use crate::parse::DownloadMode;

use crate::traits::ParseWith;

mod argument_parse;
mod impls;
mod model;
mod parse;
mod traits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let address = matches.get_one::<String>("address").unwrap();
    let browser = matches.get_one::<String>("browser").unwrap();
    let url = matches.get_one::<String>("url").unwrap();

    let download_mode = parse::DownloadMode::try_from(url.as_str()).unwrap();

    let downloads = std::env::current_dir()?.join("downloads");
    if !downloads.exists() {
        std::fs::create_dir_all(&downloads)?;
    }
    //统计运行时间
    let start = std::time::Instant::now();
    let driver = match download_mode {
        DownloadMode::Chapter(link) => {
            let driver = parse::get_driver(address, browser).await?;
            driver.goto(link).await.ok();
            let chapter = model::Chapters::parse_with(&driver).await.unwrap().unwrap();
            let mut f =
                std::fs::File::create(downloads.join(format!("{}.txt", chapter.chapters_name)))?;
            use std::io::Write;
            println!("开始下载");
            println!("正在下载: {}", chapter.chapters_name);
            f.write_all(chapter.chapters_name.as_bytes())?;
            f.write_all(b"\n")?;
            f.write_all(
                chapter
                    .chapters_content
                    .replace("Copyright 2024 69shuba.cx", "")
                    .as_bytes(),
            )?;
            driver
        }
        DownloadMode::Directory(link) => {
            let driver = parse::get_driver(address, browser).await?;
            driver.goto(link).await.ok();
            let dirctory = model::Directory::parse_with(&driver).await.unwrap();
            println!("解析完成，需要下载{}章", dirctory.inner_data.len());
            println!("开始下载");
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
            driver
        }
    };
    println!("下载完成，用时: {:?}", start.elapsed());
    driver.quit().await.ok();
    Ok(())
}

fn cli() -> clap::Command {
    clap::Command::new("shuba")
        .about("专属于69书吧的下载器 https://69shuba.cx")
        .author("Cheng")
        .version("0.1.0")
        .arg_required_else_help(true)
        .arg(
            //地址
            clap::Arg::new("address")
                .short('a')
                .long("host")
                .value_name("host")
                .help("指定一个运行了webDriver的远程主机进行抓取操作")
                .required(false)
                .default_value("http://localhost:9515"),
        )
        .arg(
            //选择浏览器
            clap::Arg::new("browser")
                .short('b')
                .help("使用的选择使用的浏览器")
                .value_parser([
                    "chrome", "chromium", "edge", "firefox", "ie", "opera", "safari",
                ])
                .default_value("edge"),
        )
        .arg(
            clap::Arg::new("url")
                .required(true)
                .short('l')
                .long("url")
                .value_name("url")
                .help("需要抓取的地址"),
        )
}
