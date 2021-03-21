use std::io;

// 1. Process `title` and `minutes` inputs.
pub fn get_inputs() -> (String, u64) {
    (get_title(), get_mins())
}

fn get_title() -> String {
    println!("What is the name of the task? (default: TASK)");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line.");
    let t = title.trim();
    if t.is_empty() {
        "TASK".to_string()
    } else {
        t.to_ascii_uppercase()
    }
}

fn get_mins() -> u64 {
    println!("How many minutes will it take?");
    let mut duration = String::new();
    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line.");
    let duration = match duration.trim().parse::<u64>() {
        Ok(num) if num == 0 => get_mins(),
        Ok(num) => num * 60,
        Err(_) => get_mins(),
    };
    duration
}

// 2. Process `number of repeats` and `recess duration` inputs.
pub fn get_recess_inputs() -> (usize, Option<u64>) {
    match get_num_of_repeats() {
        0 => (1, None),
        x => (x + 1, Some(get_recess_mins())),
    }
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
        Ok(num) if num == 0 => get_recess_mins(),
        Ok(num) => num * 60,
        Err(_) => get_recess_mins(),
    };
    duration
}
