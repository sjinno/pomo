use pomo::*;
use std::io;
use std::thread;
use std::time;

fn main() {
    do_the_thing();
}

fn do_the_thing() {
    let (title, mins) = input::get_inputs();
    let (num_of_repeats, recess_mins) = get_recess_inputs();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let mut count = 0;
    while count < num_of_repeats {
        message::begin_block(&stream_handle, &title);
        progress::update_progress(&mins);
        message::end_block(&stream_handle, &title);
        count += 1;
        if count == num_of_repeats {
            println!("DONE!! :D");
            break;
        }
        println!("Do nothing and rest.");
        thread::sleep(time::Duration::from_secs(recess_mins));
    }
}

fn get_recess_inputs() -> (usize, u64) {
    (get_num_of_repeats(), get_recess_mins())
}

fn get_num_of_repeats() -> usize {
    println!("How many times do you want to repeat?");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line.");
    match num.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => get_num_of_repeats(),
    }
}

fn get_recess_mins() -> u64 {
    println!("How many minutes do you want to rest between sessions?");
    let mut duration = String::new();
    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line.");
    let duration = match duration.trim().parse::<u64>() {
        Ok(num) => num * 60,
        Err(_) => get_recess_mins(),
    };
    duration
}
