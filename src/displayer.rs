use gstreamer::prelude::*;

use crate::media::{Content, VideoContent};

pub struct Displayer;

impl Displayer {
    pub fn play(self, content: Content) {
        match content {
            Content::Video(video_content) => self.play_video(video_content),
        }
    }

    pub fn stop() {
        todo!("to implement")
    }

    fn play_video(self, video_content: VideoContent) {
        // Initialize gstreamer
        gstreamer::init().unwrap();

        let uri = video_content
            .file
            .to_str()
            .expect("Video content file should be a valid string.");

        // Create a new playbin element, and tell it what uri to play back.
        let playbin = gstreamer::ElementFactory::make("playbin")
            .property("uri", uri)
            .build()
            .unwrap();

        // The playbin element itself is a playbin, so it can be used as one, despite being
        // created from an element factory.
        let bus = playbin.bus().unwrap();

        playbin
            .set_state(gstreamer::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");

        for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
            use gstreamer::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    break;
                }
                MessageView::StateChanged(state_changed) =>
                // We are only interested in state-changed messages from playbin
                {
                    if state_changed.src().map(|s| s == &playbin).unwrap_or(false)
                        && state_changed.current() == gstreamer::State::Playing
                    {
                        // Generate a dot graph of the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                        let bin_ref = playbin.downcast_ref::<gstreamer::Bin>().unwrap();
                        bin_ref.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PLAYING");
                    }
                }

                _ => (),
            }
        }

        playbin
            .set_state(gstreamer::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }
}
