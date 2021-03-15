use rodio::{self, OutputStreamHandle};
use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn run() {
    let (mins, title) = get_inputs();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    begin_block(&stream_handle, &title);
    update_progress(&mins);
    end_block(&stream_handle, &title);
}

fn get_inputs() -> (u64, String) {
    let mut args = env::args().skip(1);
    let mut ignore_second_arg = false;

    let mins;
    match args.next() {
        Some(m) => match m.trim().parse::<u64>() {
            Ok(num) => mins = num * 60,
            Err(_) => {
                ignore_second_arg = true;
                mins = get_mins();
            }
        },
        None => {
            ignore_second_arg = true;
            mins = get_mins();
        }
    }

    let title;
    if !ignore_second_arg {
        match args.next() {
            Some(s) => match s.is_empty() {
                true => title = get_title(),
                false => title = s,
            },
            None => title = get_title(),
        }
    } else {
        title = get_title();
    }

    (mins, title)
}

fn get_mins() -> u64 {
    println!("How many minutes are you going to study?");
    let mut duration = String::new();
    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line.");
    let duration = match duration.trim().parse::<u64>() {
        Ok(num) => num * 60,
        Err(_) => get_mins(),
    };
    duration
}

fn get_title() -> String {
    println!("What would you like to name your task? (default: Block)");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line.");
    title.trim().to_string()
}

fn begin_block(stream_handle: &OutputStreamHandle, title: &str) {
    thread::sleep(Duration::from_millis(500));
    println!("\n{} has begun.", title);
    let file = std::fs::File::open("audio/mario.wav").unwrap();
    let mario = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    mario.set_volume(0.3);
    thread::sleep(Duration::from_millis(1500));
}

fn update_progress(duration: &u64) {
    let mut count = 0;
    let mut progress = String::new();

    while &count != duration {
        if count % 60 == 0 && count != 0 {
            progress.push_str("● ");
        }
        if count % 300 == 0 && count != 0 {
            progress.push_str("| ");
        }
        print!("\r{}", progress);
        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
    progress.push('●');
    println!("\r{}", progress);
}

fn end_block(stream_handle: &OutputStreamHandle, title: &str) {
    println!("{} has ended.", title);
    let file = std::fs::File::open("audio/chime.ogg").unwrap();
    let chime = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    chime.set_volume(0.3);
    chime.sleep_until_end();
}
