use fantoccini::wd::Capabilities;
use serde_json::json;

///解析代理字符串
/// export https_proxy=http://127.0.0.1:8888;export http_proxy=http://127.0.0.1:8888;export all_proxy=socks5://127.0.0.1:8889
fn parse_proxy_caps(caps: &mut Capabilities, proxy_str: Option<&str>) -> Result<(), &'static str> {
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
    proxy_str: Option<&str>,
) -> Result<fantoccini::Client, Box<dyn std::error::Error>> {
    let mut caps = Capabilities::new();
    parse_proxy_caps(&mut caps, proxy_str)?;
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(address)
        .await
        .map_err(|e| format!("连接到WebDriver出现错误，请检查参数是否正确 {}", e).into())
}

#[derive(Debug, PartialEq, Eq, Clone)]
///在这里添加更多的下载模式解析。
pub enum DownloadMode {
    Chapter(String),
    Directory(String),
}

impl std::convert::TryFrom<&str> for DownloadMode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(mode) = is_shuba(value) {
            return Ok(mode);
        }

        Err("无法识别的下载模式")
    }
}

fn is_shuba(value: &str) -> Option<DownloadMode> {
    let argument_len = value
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .len();

    match argument_len {
        //https://69shuba.cx/txt/9958171/90560237
        5 => Some(DownloadMode::Chapter(
            value.replace("book", "txt").to_string(),
        )),
        //https://69shuba.cx/book/9958171/
        4 => Some(DownloadMode::Directory(
            value.replace("txt", "book").replace(".htm", "").to_string(),
        )),
        _ => None,
    }
}

#[allow(clippy::unnecessary_unwrap)]
pub fn parse_download_path(p: Option<&String>) -> Box<std::path::Path> {
    return if p.is_none() {
        std::env::current_dir().unwrap().join("downloads").into_boxed_path()
    } else {
        std::path::Path::new(p.unwrap()).to_path_buf().into_boxed_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        //["https:", "69shuba.cx", "txt", "9958171", "90560237"]
        let url = "https://69shuba.cx/txt/9958171/90560237";
        let result = url.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        println!("{:?}", result);
    }
    #[test]
    fn parse_chapter_mode() {
        let expected = DownloadMode::Chapter("https://69shuba.cx/txt/9958171/90560237".to_string());

        assert_eq!(
            expected,
            DownloadMode::try_from("https://69shuba.cx/book/9958171/90560237").unwrap()
        );
    }
    #[test]
    fn parse_directory_mode() {
        let expected = DownloadMode::Directory("https://69shuba.cx/book/9958171/".to_string());

        assert_eq!(
            expected,
            DownloadMode::try_from("https://69shuba.cx/txt/9958171/").unwrap()
        );
    }
}
