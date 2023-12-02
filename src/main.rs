use std::path::PathBuf;

use displayer::Displayer;
use media::{Content, VideoContent};

pub mod controller;
pub mod displayer;
pub mod media;

fn main() {
    let content = Content::Video(VideoContent {
        file: PathBuf::from(
            "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
        ),
    });
    Displayer.play(content);
    println!("coucou")
}
