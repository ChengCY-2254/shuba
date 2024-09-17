#![allow(unused_imports)]

use crate::prelude::Result;
use anyhow::Context;
#[cfg(feature = "web-driver")]
use fantoccini::wd::Capabilities;
use serde_json::{json, Value};

#[allow(clippy::unnecessary_unwrap)]
pub fn parse_download_path(p: Option<String>) -> Box<std::path::Path> {
    if p.is_none() {
        std::env::current_dir()
            .unwrap()
            .join("downloads")
            .into_boxed_path()
    } else {
        std::path::Path::new(&p.unwrap())
            .to_path_buf()
            .into_boxed_path()
    }
}

///解析代理字符串
/// export https_proxy=http://127.0.0.1:8888;export http_proxy=http://127.0.0.1:8888;export all_proxy=socks5://127.0.0.1:8889
#[cfg(feature = "web-driver")]
#[inline]
fn parse_proxy_caps(caps: &mut Capabilities, proxy_str: Option<String>) -> Result<()> {
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
            Err(crate::Error::ParseCaps(
                "不是一个有效的socks5代理字符串，请检查你的配置".to_string(),
            ))?
        };
        caps.insert("proxy".to_string(), proxy_obj);
    }
    Ok(())
}

#[cfg(feature = "web-driver")]
#[inline]
pub async fn get_driver<S: AsRef<str>>(
    address: S,
    proxy_str: Option<String>,
) -> Result<fantoccini::Client> {
    let mut caps = Capabilities::new();
    parse_proxy_caps(&mut caps, proxy_str)?;
    // parse_user_data_dir(&mut caps, user_data_dir)?;
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(address.as_ref())
        .await
        .with_context(|| "连接到WebDriver出现错误，请检查caps参数是否正确".to_string())
}

pub mod cookie {
    #![allow(clippy::enum_variant_names)]
    use fantoccini::cookies::ParseError;
    use std::borrow::Cow;
    use std::io::Write;
    use std::sync::Arc;

    /// 从路径中的文件读取cookie
    /// 一行一个cookie，就地反序列化并将其返回
    #[cfg(feature = "fantoccini")]
    pub fn read_cookies<'a, R: Iterator<Item = Cow<'a, str>>>(
        cookies_reader: R,
    ) -> Result<Vec<fantoccini::cookies::Cookie<'a>>, Error> {
        let mut cookies = vec![];
        for cookie in cookies_reader {
            let cookie = fantoccini::cookies::Cookie::parse(cookie)?;
            cookies.push(cookie);
        }
        Ok(cookies)
    }
    /// 写入cookie，一行一个
    #[cfg(feature = "fantoccini")]
    pub fn write_cookies<P: AsRef<std::path::Path>>(
        path: P,
        cookies: Vec<fantoccini::cookies::Cookie>,
    ) -> Result<(), std::io::Error> {
        let mut f = std::fs::File::create(path)?;
        for cookie in cookies {
            let cookie = cookie.encoded().to_string();
            write!(&mut f, "{}", cookie)?;
            writeln!(f)?;
        }
        Ok(())
    }
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("get a io error for parse cookies {0}")]
        IoError(#[from] std::io::Error),
        #[error("parse cookie error {0}")]
        CookieParseError(#[from] ParseError),
    }
}
