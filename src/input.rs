use std::env;
use std::io;

pub fn get_inputs() -> (u64, String) {
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
