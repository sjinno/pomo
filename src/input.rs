use std::io;

pub fn get_inputs() -> (u64, String) {
    (get_mins(), get_title())
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
