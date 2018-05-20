use std::path::PathBuf;

#[derive(Debug)]
pub struct Metadata {
    pub artist: String,
    pub title: String,
    pub audio_file: PathBuf,
}
