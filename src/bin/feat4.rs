use termion::color::{Green, Magenta, Red, White, Yellow};

// Formats string with color.
// Requires `termion` crate.
#[macro_export]
macro_rules! bold {
    ($val:expr) => {{
        use termion::style::{Bold, NoBold};
        format!("{b1}{}{b0}", $val, b1 = Bold, b0 = NoBold)
    }};
}

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

fn main() {
    // let b = bold!("Hello, world!");
    // let c = color!(Yellow, &b);
    // println!("{}", c);
    let first = color!(Green, "● ● ● ● ●", " , ");
    // let d = color!(White, "||", " ");
    let second = color!(Green, "● ● ● ● ●", " | ");
    let third = color!(Green, "● ● ● ● ●");
    println!("{}{}{}", first, second, third);
}
