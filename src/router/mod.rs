
#[derive(Debug, PartialEq, Eq, Clone)]
///在这里添加更多的下载模式解析。
pub enum Router {
    Chapter { url: String },
    Directory { url: String },
}

impl TryFrom<&str> for Router {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = value;
        #[cfg(feature = "keryo")]
        if let Some(router) = is_keryo(url) {
            return Ok(router);
        }
        #[cfg(feature = "ddxs")]
        if let Some(router) = is_ddxs(url) {
            return Ok(router);
        }
        Err(crate::Error::From("无法识别的下载模式".to_string()))?
    }
}


#[cfg(feature = "keryo")]
fn is_keryo(value: &str) -> Option<Router> {
    let args = value
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if let Some(path) = args.get(2) {
        return if path.starts_with("book") {
            //https://www.keryo.net/book_hnqjhl/ooimpn.html
            Some(Router::Chapter {
                url: value.to_string(),
            })
        } else {
            //https://www.keryo.net/xs_hnqjhl/
            Some(Router::Directory {
                url: value.to_string(),
            })
        };
    }
    log::error!("链接不匹配，无法识别下载模式");
    None
}
#[cfg(feature = "ddxs")]
fn is_ddxs(value: &str) -> Option<Router> {
    if value.ends_with(".html") {
        return Some(Router::Chapter {
            url: value.to_string(),
        });
    } else if !value
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()[2]
        .is_empty()
    {
        return Some(Router::Directory {
            url: value.to_string(),
        });
    }
    log::error!("链接不匹配，不是顶点小说网的链接");
    None
}


#[cfg(test)]
#[cfg(feature = "full")]
mod tests {
    #[test]
    #[should_panic]
    fn test_split() {
        let url = "https://www.zhihu.com/market/paid_column/1558815650394587136/section/1654884523186982912?km_channel=search&origin_label=search";
        let _args = url.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();
        panic!("len is {} {:?}", _args.len(), _args)
    }
}
