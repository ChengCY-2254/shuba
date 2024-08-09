use indicatif::style::TemplateError;
use indicatif::{ProgressBar, ProgressStyle};
use std::borrow::Cow;

pub const DEFAULT_TEMPLATE: &str = "{msg} {wide_bar} {pos}/{len} ";

///这里主要控制终端进度条的展示
pub struct Progress {
    sty: ProgressStyle,
    pb: Option<ProgressBar>,
}

impl Progress {
    pub fn new(template: &str, progress_chars: Option<&str>) -> Result<Progress, TemplateError> {
        let mut template = ProgressStyle::with_template(template)?;
        if let Some(progress_chars) = progress_chars {
            template = template.progress_chars(progress_chars);
        }

        Ok(Progress {
            sty: template,
            pb: None,
        })
    }

    pub fn start(&mut self, len: u64) {
        let pb = ProgressBar::new(len);
        pb.set_style(self.sty.clone());
        self.pb = Some(pb);
    }

    pub fn set_message(&mut self, msg: impl Into<Cow<'static, str>>) {
        if let Some(pb) = self.pb.take() {
            pb.set_message(msg);
            self.pb = Some(pb)
        }
    }

    pub fn finish_with_message(&mut self, msg: impl Into<Cow<'static, str>>) {
        if let Some(pb) = self.pb.take() {
            pb.finish_with_message(msg);
            self.pb = Some(pb)
        }
    }

    pub fn inc(&mut self, delta: u64) {
        if let Some(pb) = self.pb.take() {
            pb.inc(delta);
            self.pb = Some(pb)
        }
    }
}
