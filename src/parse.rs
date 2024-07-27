use thirtyfour::{Capabilities, DesiredCapabilities};

///从字符串中选择用来抓取的浏览器
#[inline]
fn select_browser(text: &str) -> Result<Capabilities, &'static str> {
    match text.to_lowercase().as_str() {
        "chrome" => Ok(DesiredCapabilities::chrome().into()),
        "chromium" => Ok(DesiredCapabilities::chromium().into()),
        "edge" => Ok(DesiredCapabilities::edge().into()),
        "firefox" => Ok(DesiredCapabilities::firefox().into()),
        "safari" => Ok(DesiredCapabilities::safari().into()),
        "opera" => Ok(DesiredCapabilities::opera().into()),
        "ie" => Ok(DesiredCapabilities::internet_explorer().into()),
        _ => Err("missing browser"),
    }
}
#[inline]
pub async fn get_driver(
    address: &str,
    browser: &str,
) -> Result<thirtyfour::WebDriver, &'static str> {
    let capabilities = select_browser(browser)?;
    Ok(thirtyfour::WebDriver::new(address, capabilities)
        .await
        .unwrap())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DownloadMode {
    Chapter(String),
    Directory(String),
}

impl std::convert::TryFrom<&str> for DownloadMode {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let argument_len = value
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .len();

        match argument_len {
            //https://69shuba.cx/txt/9958171/90560237
            5 => Ok(DownloadMode::Chapter(
                value.replace("book", "txt").to_string(),
            )),
            //https://69shuba.cx/book/9958171/
            4 => Ok(DownloadMode::Directory(
                value.replace("txt", "book").replace(".htm","").to_string(),
            )),
            _ => Err("parse url error please check out your url"),
        }
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
