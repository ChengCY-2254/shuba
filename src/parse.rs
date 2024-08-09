use crate::model::CliArguments;
use fantoccini::wd::Capabilities;
use serde_json::json;

#[derive(Debug, PartialEq, Eq, Clone)]
///在这里添加更多的下载模式解析。
pub enum DownloadMode {
    Chapter { url: String },
    Directory { url: String },
}

impl std::convert::TryFrom<&CliArguments> for DownloadMode {
    type Error = &'static str;

    fn try_from(value: &CliArguments) -> Result<Self, Self::Error> {
        let url = &value.url;
        #[cfg(feature = "shuba")]
        if let Some(mode) = is_shuba(url) {
            return Ok(mode);
        }
        #[cfg(feature = "keryo")]
        if let Some(mode) = is_keryo(url) {
            return Ok(mode);
        }

        Err("无法识别的下载模式")
    }
}

#[cfg(feature = "shuba")]
fn is_shuba(value: &str) -> Option<DownloadMode> {
    let argument_len = value
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .len();

    match argument_len {
        //https://69shuba.cx/txt/9958171/90560237
        5 => Some(DownloadMode::Chapter{
            url:value.replace("book", "txt").to_string(),
        })
        ,
        //https://69shuba.cx/book/9958171/
        4 => Some(DownloadMode::Directory{
            url:value.replace("txt", "book").replace(".htm", "").to_string(),
        }),
        _ => None,
    }
}
#[cfg(feature = "keryo")]
fn is_keryo(value: &str) -> Option<DownloadMode> {
    let args = value
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if let Some(path) = args.get(2) {
        return if path.starts_with("book") {
            //https://www.keryo.net/book_hnqjhl/ooimpn.html
            Some(DownloadMode::Chapter {
                url: value.to_string(),
            })
        } else {
            //https://www.keryo.net/xs_hnqjhl/
            Some(DownloadMode::Directory {
                url: value.to_string(),
            })
        };
    }
    log::error!("链接不匹配，无法识别下载模式");
    None
}
#[allow(clippy::unnecessary_unwrap)]
pub fn parse_download_path(p: Option<String>) -> Box<std::path::Path> {
    return if p.is_none() {
        std::env::current_dir()
            .unwrap()
            .join("downloads")
            .into_boxed_path()
    } else {
        std::path::Path::new(&p.unwrap())
            .to_path_buf()
            .into_boxed_path()
    };
}
#[cfg(test)]
#[cfg(feature = "full")]
mod tests {
    #[test]
    fn test_split() {
        let url = "https://www.keryo.net/book_hnqjhl/ooimpn.html";
        let _args = url.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        // panic!("{:?}", _args)
    }
}

///解析代理字符串
/// export https_proxy=http://127.0.0.1:8888;export http_proxy=http://127.0.0.1:8888;export all_proxy=socks5://127.0.0.1:8889
fn parse_proxy_caps(
    caps: &mut Capabilities,
    proxy_str: Option<String>,
) -> Result<(), &'static str> {
    if let Some(proxy_str) = proxy_str {
        //socks5代理
        let proxy_obj = if proxy_str.starts_with("socks5://") {
            let proxy_str = proxy_str.replace("socks5://", "");
            json!({
                "proxyType": "manual",
                "socksProxy": proxy_str,
                "socksVersion":5
            })
        } else {
            eprintln!("不是一个有效的socks5代理字符串，请检查你的配置");
            std::process::exit(1);
        };
        caps.insert("proxy".to_string(), proxy_obj);
    }
    Ok(())
}
#[inline]
pub async fn get_driver(
    address: &str,
    proxy_str: Option<String>,
) -> Result<fantoccini::Client, Box<dyn std::error::Error>> {
    let mut caps = Capabilities::new();
    parse_proxy_caps(&mut caps, proxy_str)?;
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(address)
        .await
        .map_err(|e| format!("连接到WebDriver出现错误，请检查参数是否正确 {}", e).into())
}
#[cfg(feature = "unstable")]
mod format {
    use clap::builder::PossibleValue;
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Format {
        Txt,
        Epub,
    }

    impl std::convert::TryFrom<&str> for Format {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "txt" => Ok(Format::Txt),
                "epub" => Ok(Format::Epub),
                _ => Err("无法识别的格式"),
            }
        }
    }

    impl clap::ValueEnum for Format {
        fn value_variants<'a>() -> &'a [Self] {
            &[Format::Txt, Format::Epub]
        }

        fn to_possible_value(&self) -> Option<PossibleValue> {
            match self {
                Format::Txt => Some(PossibleValue::new("txt").help(".txt格式")),
                Format::Epub => Some(PossibleValue::new("epub").help(".epub格式")),
            }
        }
    }

    impl std::str::FromStr for Format {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "txt" => Ok(Format::Txt),
                "epub" => Ok(Format::Epub),
                _ => Err("无法识别的格式".to_string()),
            }
        }
    }

    impl Default for Format {
        fn default() -> Self {
            Self::Txt
        }
    }
}
