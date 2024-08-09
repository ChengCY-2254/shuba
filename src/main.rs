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

    if arguments.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    let url = arguments.url.clone();
    let handler = handler::Handlers::try_from(url.as_ref())?;
    let download_mode = parse::DownloadMode::try_from(&arguments).unwrap();
    
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
