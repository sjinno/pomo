use std::io::Write;
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
    // Duration is in secs.
    // e.g. 1 min -> 60 secs
    while &count != duration {
        if count != 0 {
            match count {
                five_mins if five_mins % 300 == 0 => {
                    let color = color!(Green, "●", " | ");
                    progress = progress.replace("● ● ● ● ● ", &color);
                }
                one_min if one_min % 60 == 0 => {
                    let color = color!(Green, "●", " ");
                    progress = progress.replace("● ● ● ● ● ", &color);
                }
                ten_secs if ten_secs % 10 == 0 => progress.push_str("● "),
                _ => (),
            }
        }
        print!("\r{clr}{}", progress, clr = clear::CurrentLine,);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
    let color = color!(Green, "●");
    progress = progress.replace("● ● ● ● ● ", &color);
    println!("\r{clr}{}", progress, clr = clear::CurrentLine);
}
