mod impls;
mod model;
mod parse;
mod traits;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();
    let address = matches.get_one::<String>("address").unwrap();
    let url = matches.get_one::<String>("url").unwrap();
    let check_proxy = matches.get_flag("check_proxy");
    let proxy_str: Option<&str> = matches
        .get_one::<String>("proxy_address")
        .map(String::as_ref);

    let download_mode = parse::DownloadMode::try_from(url.as_str()).unwrap();

    let downloads = std::env::current_dir()?.join("downloads");
    if !downloads.exists() {
        std::fs::create_dir_all(&downloads)?;
    }
    //统计运行时间
    let start = std::time::Instant::now();
    download_mode
        .run(address, &downloads, check_proxy, proxy_str)
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
        // .arg(
        //     //选择浏览器
        //     clap::Arg::new("browser")
        //         .short('b')
        //         .help("使用的选择使用的浏览器")
        //         .value_parser([
        //             "chrome", "chromium", "edge", "firefox", "ie", "opera", "safari",
        //         ])
        //         .default_value("edge"),
        // )
        .arg(
            clap::Arg::new("url")
                .required(true)
                .short('l')
                .long("url")
                .value_name("url")
                .help("需要抓取的地址"),
        )
        .arg(
            clap::Arg::new("check_proxy")
                .required(false)
                .short('c')
                .action(clap::ArgAction::SetFalse)
                .help("是否跳过代理检查"),
        )
        .arg(
            clap::Arg::new("proxy_address")
                .required(false)
                .long("proxy")
                // .help("让浏览器通过代理进行网页访问，建议使用socks5代理，如果使用http/https代理，请使用;将两个url分割开来")
                .help("让浏览器通过代理进行网页访问，使用socks5代理")
                .value_name("proxy_address"),
        )
}
