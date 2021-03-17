use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::color::Green;

// Formats string with color.
// Requires `termion` crate.
#[macro_export]
macro_rules! color {
    ($clr:expr, $val:expr) => {{
        use termion::color::{Fg, Reset};
        format!("{c1}{}{c0}", $val, c1 = Fg($clr), c0 = Fg(Reset))
    }};

    ($clr:expr, $val:expr, $opt:expr) => {{
        use termion::color::{Fg, Reset};
        format!("{c1}{}{c0}{}", $val, $opt, c1 = Fg($clr), c0 = Fg(Reset))
    }};
}

pub fn update_progress(duration: &u64) {
    let mut count = 1;
    let mut progress = String::new();
    let one_min_passed = |s: String, c: &str| -> String { s.replace("● ● ● ● ● ", &c) };
    // Duration is in secs.
    // e.g. 1 min -> 60 secs
    while &count != duration {
        if count != 0 {
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
