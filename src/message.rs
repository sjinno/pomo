use chrono::Local;
use rodio::{self, OutputStreamHandle};
use std::io;
use std::thread;
use std::time::Duration;

pub fn begin_block(stream_handle: &OutputStreamHandle, title: &str) {
    thread::sleep(Duration::from_millis(500));
    println!("{} has started. [{}]", title, stamp_time());
    let file = std::fs::File::open("audio/mario.wav").unwrap();
    let mario = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    mario.set_volume(0.2);
    thread::sleep(Duration::from_millis(2000));
}

pub fn end_block(stream_handle: &OutputStreamHandle, title: &str) {
    println!("{} has ended. [{}]", title, stamp_time());
    let file = std::fs::File::open("audio/chime.ogg").unwrap();
    let chime = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    chime.set_volume(0.3);
    chime.sleep_until_end();
}

pub fn do_nothing(recess_mins: &u64) {
    println!("Do nothing and rest.");
    thread::sleep(Duration::from_secs(*recess_mins));
}

fn stamp_time() -> String {
    let curr_time = Local::now();
    curr_time.format("%H:%M:%S").to_string()
}
