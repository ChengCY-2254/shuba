use crate::model::CliArguments;
use crate::prelude::{Result,Error};
use log::info;
use std::io::stdin;
use anyhow::Context;

mod build_info;
mod cli;
mod handler;
mod handlers;
mod impls;
mod macros;
mod model;
mod parse;
mod router;
mod traits;
mod utils;
mod errors;
mod prelude {
    pub use crate::errors::Error;
    #[cfg(feature = "web-driver")]
    pub type Driver = fantoccini::Client;
    #[cfg(feature = "web-driver")]
    pub type By<'a> = fantoccini::Locator<'a>;
    // pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    pub type Result<T> = anyhow::Result<T>;
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    #[cfg(all(feature = "debug", feature = "env_logger"))]
    env_logger::init();
    let matches = cli::cli().get_matches();

    let arguments = CliArguments::from(matches);

    #[cfg(all(feature = "debug", feature = "env_logger"))]
    if arguments.debug {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }

    if arguments.print_support {
        log::info!("查看受支持的网站");
        let handle_list = handler::Handlers::values();
        if handle_list.is_empty() {
            println!("什么都没有(O_o)??");
            std::process::exit(0);
        }
        handle_list
            .iter()
            .for_each(|web_site_name| println!("{web_site_name}"));
        std::process::exit(0);
    }
    if arguments.pre_login {
        return pre_login_logic(&arguments).await;
    }

    download_logic(&arguments).await
}
///小说下载逻辑
async fn download_logic(arguments: &CliArguments) -> Result<()> {
    let url = arguments.url.clone().expect("请提供URL参数");
    let handler = handler::Handlers::try_from(url.as_ref())?;
    let download_mode = router::Router::try_from(url.as_ref()).unwrap();

    let downloads = parse::parse_download_path(arguments.download_path.clone());
    if !downloads.exists() {
        info!("创建路径 {}", downloads.display());
        std::fs::create_dir_all(&downloads)?;
    }
    //统计运行时间
    let start = std::time::Instant::now();
    handler
        .run(arguments.clone(), download_mode, &downloads)
        .await
        .with_context(||{"下载内容时出现错误"})?;
    println!("下载完成，用时: {:?}", start.elapsed());
    Ok(())
}
///预登录逻辑
async fn pre_login_logic(arguments: &CliArguments) -> Result<()> {
    let cookie_path = arguments.user_data_dir.clone().unwrap();
    println!("请在浏览器中打开对应网页进行登录，登录完成后返回终端点按回车");
    let driver = parse::get_driver(&arguments.address, arguments.proxy_str.clone()).await?;
    driver.set_window_size(1109, 797).await.ok();
    //阻塞住，让用户登录
    stdin().read_line(&mut String::new())?;
    //默认登录完毕，写入cookie然后推出driver
    let cookies = driver.get_all_cookies().await?;
    let display_path = std::env::current_dir()?.join(&cookie_path);
    info!("正在往{}回写cookie", display_path.display());
    println!("正在往{}回写cookie", display_path.display());
    parse::cookie::write_cookies(cookie_path, cookies)?;
    driver.close().await.ok();
    Ok(())
}
