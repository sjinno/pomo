use rodio::{self, OutputStreamHandle};
use std::io;
use std::thread;
use std::time::Duration;

pub fn begin_block(stream_handle: &OutputStreamHandle, title: &str) {
    thread::sleep(Duration::from_millis(500));
    println!("{} has begun.", title);
    let file = std::fs::File::open("audio/mario.wav").unwrap();
    let mario = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    mario.set_volume(0.3);
    thread::sleep(Duration::from_millis(1500));
}

pub fn end_block(stream_handle: &OutputStreamHandle, title: &str) {
    println!("{} has ended.", title);
    let file = std::fs::File::open("audio/chime.ogg").unwrap();
    let chime = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    chime.set_volume(0.3);
    chime.sleep_until_end();
}
