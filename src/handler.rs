use crate::model::CliArguments;
use log::info;
use crate::router::Router;

#[derive(proc_macro_workshop::Enums)]
pub enum Handlers {
    #[cfg(feature = "shuba")]
    Shuba(crate::handlers::Shuba),
    #[cfg(feature = "keryo")]
    Keryo(crate::handlers::Keryo),
    #[cfg(feature = "ddxs")]
    Ddxs(crate::handlers::Ddxs),
    #[cfg(feature = "zhihu")]
    Zhihu(crate::handlers::Zhihu)
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
        #[cfg(feature = "zhihu")]
        if value.starts_with("https://www.zhihu.com/") {
            info!("选择解析器：zhihu");
            return Ok(Handlers::Zhihu(crate::handlers::Zhihu));
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
        mode: Router,
        download_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::traits::Run;
        let address: &str = cli_arguments.address.as_str();
        let proxy_str = cli_arguments.proxy_str;
        let speed = cli_arguments.speed;
        let user_data_dir = cli_arguments.user_data_dir;
        // let format = cli_arguments.format;
        //todo 用宏将这里简化一下运行
        match self {
            #[cfg(feature = "shuba")]
            Handlers::Shuba(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed,user_data_dir)
                    .await
            }
            #[cfg(feature = "keryo")]
            Handlers::Keryo(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed,user_data_dir)
                    .await
            }
            #[cfg(feature = "ddxs")]
            Handlers::Ddxs(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed,user_data_dir)
                    .await
            }
            #[cfg(feature = "zhihu")]
            Handlers::Zhihu(handle)=>{
                handle
                    .run(address, download_path, proxy_str, mode, speed,user_data_dir)
                    .await
            }
            _ => Err("未找到与域名对应的下载器".into()),
        }
    }
}
