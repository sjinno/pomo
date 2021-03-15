use std::io;

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
        Ok(num) => num * 60,
        Err(_) => get_mins(),
    };
    duration
}
