use std::io;

pub fn get_inputs() -> (String, u64) {
    (get_title(), get_mins())
}

fn get_title() -> String {
    println!("What is the name of the task? (default: Block)");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line.");
    title.trim().to_string()
}

fn get_mins() -> u64 {
    println!("How many minutes is it going to take?");
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
