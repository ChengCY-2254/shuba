use log::info;

pub enum Handlers {
    #[cfg(feature = "shuba")]
    //需要把枚举值全部打印出来
    Shuba(crate::handlers::Shuba),
}

impl std::convert::TryFrom<&str> for Handlers {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[cfg(feature = "shuba")]
        if value.starts_with("https://69shuba.cx") {
            info!("选择解析器：69shuba");
            return Ok(Handlers::Shuba(crate::handlers::Shuba));
        }

        Err("未找到与域名对应的解析器")
    }
}

impl Handlers {
    #[allow(unreachable_patterns)]
    pub async fn run(
        self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode,
        speed: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::traits::Run;

        match self {
            #[cfg(feature = "shuba")]
            Handlers::Shuba(handle) => handle.run(address, download_path, proxy_str, mode,speed).await,
            _ => Err("未找到与域名对应的下载器".into()),
        }
    }
}
