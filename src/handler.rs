use crate::handlers::Shuba;
use crate::traits:: Run;

pub enum Handlers {
    Shuba(Shuba),
}

impl std::convert::TryFrom<&str> for Handlers {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("https://69shuba.cx") {
            return Ok(Handlers::Shuba(Shuba));
        }

        Err("未找到与域名对应的下载器")
    }
}

impl Handlers {
    #[allow(unreachable_patterns)]
    pub async fn run(
        &self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Handlers::Shuba(handle) => handle.run(address, download_path, proxy_str, mode).await,
            _ => Err("未找到与域名对应的下载器".into()),
        }
    }
}
