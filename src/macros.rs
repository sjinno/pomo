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

#[macro_export]
macro_rules! bold {
    ($val:expr) => {{
        use termion::style::{Bold, Reset};
        format!("{b1}{}{b0}", $val, b1 = Bold, b0 = Reset)
    }};
}
