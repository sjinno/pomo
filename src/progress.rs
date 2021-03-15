use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn update_progress(duration: &u64) {
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
