#![allow(unused_imports)]

mod progress;

pub mod text;

pub mod format;

pub use progress::Progress;
use progress::DEFAULT_TEMPLATE;

pub fn default_progress() -> Progress {
    Progress::new(DEFAULT_TEMPLATE, Some("##-")).unwrap()
}
#[inline]
pub fn seconds_to_millis(seconds: Option<f32>) -> Option<std::time::Duration> {
    seconds.map(|seconds| std::time::Duration::from_millis((seconds * 1000.) as u64))
}
