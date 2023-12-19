// #[derive(Debug)]

mod media;

use media::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Playing audio: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Playing video: {}", self.0);
    }
}

fn main() {
    println!("Super player!");
    let audio = Audio(String::from("ambient_music.mp3"));
    let video = Video(String::from("big_buck_bunny.mkv"));

    audio.play();
    video.play();
}
