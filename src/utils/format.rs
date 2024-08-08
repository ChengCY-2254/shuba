#![allow(unused)]
use crate::model::Chapter;

pub fn write_chapter_by_txt(
    chapter: Chapter,
    file: &mut std::fs::File,
) -> Result<(), std::io::Error> {
    use std::io::Write;
    writeln!(file, "{}", chapter)
}
pub fn write_chapter_epub(_chapter: Chapter, _file: &mut std::fs::File) {
    unimplemented!()
}
pub fn write_chapters_by_epub(_chapters: Vec<Chapter>, _file: &mut std::fs::File) {
    unimplemented!()
}
