use crate::model::CliArguments;
use crate::router::Router;
use log::info;
use crate::prelude::*;

#[derive(proc_macro_workshop::Enums)]
pub enum Handlers {
    // #[cfg(feature = "shuba")]
    // Shuba(crate::handlers::Shuba),
    #[cfg(feature = "keryo")]
    Keryo(crate::handlers::Keryo),
    #[cfg(feature = "ddxs")]
    Ddxs(crate::handlers::Ddxs),
}

impl std::convert::TryFrom<&str> for Handlers {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
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
        Err("未找到与域名对应的解析器").map_err(|e|crate::Error::From(e.to_string()))?
    }
}

impl Handlers {
    #[allow(unreachable_patterns)]
    pub async fn run(
        self,
        cli_arguments: CliArguments,
        mode: Router,
        download_path: &std::path::Path,
    ) -> Result<()> {
        use crate::traits::Run;
        let address: &str = cli_arguments.address.as_str();
        let proxy_str = cli_arguments.proxy_str;
        let speed = cli_arguments.speed;
        let user_data_file = cli_arguments.user_data_dir;
        match self {
            #[cfg(feature = "keryo")]
            Handlers::Keryo(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed, user_data_file)
                    .await
            }
            #[cfg(feature = "ddxs")]
            Handlers::Ddxs(handle) => {
                handle
                    .run(address, download_path, proxy_str, mode, speed, user_data_file)
                    .await
            }
            _ => Err(crate::Error::Handler("未找到与域名对应的下载器".to_string()))?,
        }
    }
}
mod __private{
    #![allow(unused)]
    //! 文件配置读取，将url放置到外部定义，以免域名频繁变更
    //! todo
    use std::sync::LazyLock;

    pub(crate) static ENV_FILE: LazyLock<Option<String>> = LazyLock::new(||{
        let env_file = std::env::current_dir()
             .map(|path|path.join(".env"))
             .map(std::fs::read_to_string);
        match env_file{
            Ok(Ok(file))=>Some(file),
            _=>None
        }
    });

    fn get_url<'a>(_key:&'a str ,default_value:&'a str)->String{
        match ENV_FILE.as_ref() {
            None => {default_value.to_string()}
            Some(file) => {
                let _line_iter = file.lines();
            // let a =    file
            //         .lines()
            //         .filter(|line| line.starts_with(key))
            //         .map(|line|{
            //             let mut iter =line.split('=');
            //             let _ = iter.next();
            //             iter.next()
            //         })};

            unimplemented!()
            }

        }
    }
}