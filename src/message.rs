use std::io;
use std::thread;
use std::time::Duration;

use chrono::Local;
use rodio::{self, OutputStreamHandle};
use termion::color::Yellow;

use crate::{bold, color};

pub fn begin_block(stream_handle: &OutputStreamHandle, title: &str) {
    thread::sleep(Duration::from_millis(500));
    println!("{} has started. [{}]", title, bold!(stamp_time()));
    play_starting_bell(stream_handle);
}

pub fn end_block(
    stream_handle: &OutputStreamHandle,
    title: &str,
    recess_mins: &u64,
    is_done: bool,
) {
    println!("{} has ended. [{}]", title, bold!(stamp_time()));
    if is_done {
        println!("{}", bold!(color!(Yellow, "★★★ ALL DONE!! ★★★")));
        play_ending_bell(stream_handle);
    } else {
        println!("{}", color!(Yellow, "Do nothing and rest."));
        play_ending_bell(stream_handle);
        thread::sleep(Duration::from_secs(*recess_mins - 12));
    }
}

fn play_starting_bell(stream_handle: &OutputStreamHandle) {
    let file = std::fs::File::open("audio/mario.wav").unwrap();
    let mario = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    mario.set_volume(0.2);
    thread::sleep(Duration::from_millis(2000));
}

fn play_ending_bell(stream_handle: &OutputStreamHandle) {
    let file = std::fs::File::open("audio/chime.ogg").unwrap();
    let chime = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    chime.set_volume(0.3);
    thread::sleep(Duration::from_secs(12));
}

fn stamp_time() -> String {
    let curr_time = Local::now();
    curr_time.format("%H:%M:%S").to_string()
}
