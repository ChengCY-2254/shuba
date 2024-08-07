mod handler;
mod handlers;
mod impls;
mod model;
mod parse;
mod traits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let address = matches.get_one::<String>("address").unwrap();
    let url = matches.get_one::<String>("url").unwrap();
    let proxy_str: Option<&str> = matches
        .get_one::<String>("proxy_address")
        .map(String::as_ref);
    let download_path: Option<&String> = matches.get_one("download_path");
    let sleed:Option<f32> = matches.get_one("speed").map(|str:&String|str.parse::<f32>().unwrap());
    
    
    let handler = handler::Handlers::try_from(url.as_str())?;
    let download_mode = parse::DownloadMode::try_from(url.as_str()).unwrap();

    let downloads = parse::parse_download_path(download_path);
    if !downloads.exists() {
        std::fs::create_dir_all(&downloads)?;
    }
    //统计运行时间
    let start = std::time::Instant::now();
    handler
        .run(address, &downloads, proxy_str, download_mode,sleed)
        .await
        .map_err(|e| format!("下载时出现错误 {:?}", e))?;
    println!("下载完成，用时: {:?}", start.elapsed());
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
            clap::Arg::new("url")
                .required(true)
                .short('l')
                .long("url")
                .value_name("url")
                .help("需要抓取的地址"),
        )
        .arg(
            clap::Arg::new("proxy_address")
                .required(false)
                .long("proxy")
                .help("让浏览器通过代理进行网页访问，使用socks5代理")
                .value_name("proxy_address"),
        )
        .arg(
            clap::Arg::new("download_path")
                .required(false)
                .long("path")
                .short('p')
                .help("指定下载路径，默认为当前目录下的downloads文件夹"),
        )
        .arg(
            clap::Arg::new("speed")
                .long("speed")
                .required(false)
                .help("抓取间隔，默认不限制。单位为秒")
            ,
        )
}
