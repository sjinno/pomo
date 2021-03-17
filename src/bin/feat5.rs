use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Hey there.").unwrap();
}
