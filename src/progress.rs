use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use termion::clear;
use termion::color::Green;

use crate::color;

pub fn update_progress(duration: &u64) {
    let mut count = 2;
    let mut progress = String::new();
    let one_min_passed = |s: String, c: &str| -> String { s.replace("● ● ● ● ● ", &c) };
    // Duration is in secs.
    // e.g. 1 min -> 60 secs
    while &count != duration {
        match count {
            ten_mins if ten_mins % 600 == 0 => {
                let color = color!(Green, "●", " || ");
                progress = one_min_passed(progress, &color);
            }
            five_mins if five_mins % 300 == 0 => {
                let color = color!(Green, "●", " | ");
                progress = one_min_passed(progress, &color);
            }
            one_min if one_min % 60 == 0 => {
                let color = color!(Green, "●", " ");
                progress = one_min_passed(progress, &color);
            }
            ten_secs if ten_secs % 10 == 0 => progress.push_str("● "),
            _ => (),
        }
        print!("\r{clr}{}", progress, clr = clear::CurrentLine,);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
    let color = color!(Green, "●");
    progress = one_min_passed(progress, &color);
    println!("\r{clr}{}", progress, clr = clear::CurrentLine);
}
