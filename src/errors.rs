//! 错误定义


#[derive(thiserror::Error,Debug)]
pub enum Error{
    #[error("Get an Io Error ${source:?}")]
    Io {
        #[from] source:std::io::Error
    },
    #[error("From convert Error ${0}")]
    From(String),
    #[error("url handler get an err ${0} ")]
    Handler(String),
    #[error("parse caps get an error ${0}")]
    ParseCaps(String),
    #[error("parse html element get an error ${0}")]
    ParseElement(String),
}