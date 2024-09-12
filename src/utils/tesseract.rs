#![cfg(feature = "tesseract")]

pub struct OCR;

impl OCR{
    pub fn scan(){
        let _ = tesseract::Tesseract::new_with_data(&vec![],Some("zh-cn"),tesseract::OcrEngineMode::LstmOnly);
    }
}