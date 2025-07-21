use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

const START_SOUND_FILE: &'static str = "assets/go-go-go.mp3";
const END_SOUND_FILE: &'static str = "assets/go-go-go.mp3";
const FINISH_SOUND_FILE: &'static str = "assets/go-go-go.mp3";

pub struct Conductor {
    stream_handle: OutputStream,
    start: BufReader<std::fs::File>,
    playing: bool,
}

impl Conductor {
    pub fn new() -> Self {
        let start = BufReader::new(File::open(START_SOUND_FILE).unwrap());
        Self {
            stream_handle: rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio stream"),
            start,
            playing: false,
        }
    }

    pub fn play(&mut self) {
        if self.playing {
            return;
        }

        println!("Playing!");
        let file = BufReader::new(File::open(START_SOUND_FILE).unwrap());
        let sink = rodio::play(&self.stream_handle.mixer(), file).unwrap();

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        // std::thread::sleep(std::time::Duration::from_secs(5));
        self.playing = true;
    }
}
