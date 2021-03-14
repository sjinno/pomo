use rodio::{self, OutputStreamHandle};
use std::env;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let arg;
    match env::args().nth(1) {
        Some(num) => match num.trim().parse::<u64>() {
            Ok(n) => arg = n * 60,
            Err(_) => arg = get_input(),
        },
        None => arg = get_input(),
    };
    println!("{}", arg);
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    begin_block(&stream_handle);
    thread::sleep(Duration::from_secs(arg));
    end_block(&stream_handle);
}

fn get_input() -> u64 {
    println!("How many minutes are you going to study?");
    let mut duration = String::new();
    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line.");
    let duration = match duration.trim().parse::<u64>() {
        Ok(num) => num * 60,
        Err(_) => get_input(),
    };
    duration
}

fn begin_block(stream_handle: &OutputStreamHandle) {
    thread::sleep(Duration::from_millis(500));
    println!("New block has begun.");
    let file = std::fs::File::open("audio/mario.wav").unwrap();
    let mario = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    mario.set_volume(0.3);
    thread::sleep(Duration::from_millis(1500));
}

fn end_block(stream_handle: &OutputStreamHandle) {
    println!("Block has ended.");
    let file = std::fs::File::open("audio/chime.ogg").unwrap();
    let chime = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    chime.set_volume(0.3);
    chime.sleep_until_end();
}
