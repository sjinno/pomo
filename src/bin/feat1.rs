use std::{io::Write, thread};
use std::{ops::Add, time};

fn main() {
    // print!("Hello");
    // std::io::stdout().flush().unwrap(); // WTH Lol
    // thread::sleep(time::Duration::from_secs(2));
    // println!(", world!");

    let d1 = time::Duration::new(2, 0);
    let mut d2 = time::Duration::new(0, 0);
    while d2 != d1 {
        thread::sleep(time::Duration::from_secs(1));
        d2 += time::Duration::from_secs(1);
        // d2 = d2.add(time::Duration::from_secs(1));
        println!("{:?}", d2);
    }
}
