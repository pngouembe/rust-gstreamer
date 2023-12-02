use std::path::PathBuf;

pub enum Content {
    Video(VideoContent),
}

pub struct VideoContent {
    pub file: PathBuf,
}
