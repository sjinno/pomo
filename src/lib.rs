pub mod input;
pub mod message;
pub mod progress;

pub fn run() {
    do_the_thing();
}

fn do_the_thing() {
    let (title, mins) = input::get_inputs();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    message::begin_block(&stream_handle, &title);
    progress::update_progress(&mins);
    message::end_block(&stream_handle, &title);
}
