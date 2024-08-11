use crate::build_info;

const VERSION: &str = "0.2.2";

fn build_long_about() -> String {
    format!(
        r#"
小说下载器 暂时仅支持 [69书吧:https://69shuba.cx]
仓库地址: https://github.com/ChengCY-2254/shuba.git
构建时间: {}
构建id: {}
        "#,
        build_info::BUILD_TIME,
        build_info::GIT_HASH_7,
    )
    .trim()
    .to_string()
}

fn about() -> String {
    format!(
        r#"
仓库地址 https://github.com/ChengCY-2254/shuba.git
构建时间: {}
构建id: {}
    "#,
        build_info::BUILD_TIME,
        build_info::GIT_HASH_7
    )
    .trim()
    .to_string()
}

// fn build_handler_msg(){}
pub fn cli() -> clap::Command {
    #[allow(unused_mut)]
    let mut cli = clap::Command::new("shuba")
        .long_about(build_long_about())
        .about(about())
        .author("Cheng")
        .version(VERSION)
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
                .required(false)
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
                .help("抓取间隔，默认不限制。单位为秒"),
        )
        .arg(
            clap::Arg::new("support_web_site")
                .long("support")
                .help("查看支持的网站")
                .action(clap::ArgAction::SetTrue),
        );
    
    #[cfg(feature = "debug")]
    {
        cli = cli.arg(
        clap::Arg::new("debug")
            .long("debug")
            .help("设置debug模式，打印更多调试信息")
            .required(false)
            .action(clap::ArgAction::SetTrue),
    )
    }

    #[cfg(feature = "unstable")]
    {
        use crate::parse::Format;
        use clap::value_parser;
        cli = cli.arg(
            clap::Arg::new("download_format")
                .long("format")
                .value_parser(value_parser!(Format))
                .help("指定下载格式，默认为txt"),
        );
    }
    cli
}
