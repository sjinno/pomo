mod macros;

pub mod input;
pub mod message;
pub mod progress;

use termion::color::Yellow;

pub fn run() {
    do_the_thing();
}

fn do_the_thing() {
    let (title, mins) = input::get_inputs();
    let (num_of_repeats, recess_mins) = input::get_recess_inputs();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let mut count = 0;
    loop {
        message::begin_block(&stream_handle, &title);
        progress::update_progress(&mins);
        message::end_block(&stream_handle, &title);
        count += 1;
        if count == num_of_repeats {
            println!("{}", bold!(color!(Yellow, "★★★ ALL DONE!! ★★★")));
            break;
        }
        message::do_nothing(&recess_mins);
    }
}
