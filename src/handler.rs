use crate::model::CliArguments;
use crate::parse::DownloadMode;
use log::info;

#[derive(proc_macro_workshop::Enums)]
pub enum Handlers {
    #[cfg(feature = "shuba")]
    Shuba(crate::handlers::Shuba),
    #[cfg(feature = "keryo")]
    Keryo(crate::handlers::Keryo),
    #[cfg(feature = "ddxs")]
    Ddxs(crate::handlers::Ddxs),
}

impl std::convert::TryFrom<&str> for Handlers {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[cfg(feature = "shuba")]
        if value.starts_with("https://69shuba.cx") {
            info!("选择解析器：69shuba");
            return Ok(Handlers::Shuba(crate::handlers::Shuba));
        }
        #[cfg(feature = "keryo")]
        if value.starts_with("https://www.keryo.net") {
            info!("选择解析器：keryo");
            return Ok(Handlers::Keryo(crate::handlers::Keryo));
        }
        #[cfg(feature = "ddxs")]
        if value.starts_with("https://www.ddxs.com") {
            info!("选择解析器：ddxs");
            return Ok(Handlers::Ddxs(crate::handlers::Ddxs));
        }
        info!("未找到解析器");
        Err("未找到与域名对应的解析器")
    }
}

impl Handlers {
    #[allow(unreachable_patterns)]
    pub async fn run(
        self,
        cli_arguments: CliArguments,
        mode: DownloadMode,
        download_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::traits::Run;
        let address: &str = cli_arguments.address.as_str();
        let proxy_str = cli_arguments.proxy_str;
        let speed = cli_arguments.speed;
        // let format = cli_arguments.format;
        match self {
            #[cfg(feature = "shuba")]
            Handlers::Shuba(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed)
                    .await
            }
            #[cfg(feature = "keryo")]
            Handlers::Keryo(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed)
                    .await
            }
            #[cfg(feature = "ddxs")]
            Handlers::Ddxs(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed)
                    .await
            }
            _ => Err("未找到与域名对应的下载器".into()),
        }
    }
}
