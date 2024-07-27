use async_trait::async_trait;
use thirtyfour::WebDriver;

#[async_trait]
pub trait ParseWith<T = WebDriver> {
    type Output;
    async fn parse_with(_driver: &'_ T) -> Result<Self::Output, Box<dyn std::error::Error + Send>>{
        unimplemented!("Please implement this trait for your type")
    }
}
