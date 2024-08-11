use crate::model::CliArguments;

mod build_info;
mod cli;
mod handler;
mod handlers;
mod impls;
mod model;
mod parse;
mod traits;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "env_logger")]
    env_logger::init();
    let matches = cli::cli().get_matches();

    let arguments = CliArguments::from(matches);
    
    #[cfg(feature = "env_logger")]
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

    let url = arguments.url.clone().expect("请提供URL参数");
    let handler = handler::Handlers::try_from(url.as_ref())?;
    let download_mode = parse::DownloadMode::try_from(url.as_ref()).unwrap();

    let downloads = parse::parse_download_path(arguments.download_path.clone());
    if !downloads.exists() {
        std::fs::create_dir_all(&downloads)?;
    }
    //统计运行时间
    let start = std::time::Instant::now();
    handler
        .run(arguments, download_mode, &downloads)
        .await
        .map_err(|e| format!("下载时出现错误 {:?}", e))?;
    println!("下载完成，用时: {:?}", start.elapsed());
    Ok(())
}
