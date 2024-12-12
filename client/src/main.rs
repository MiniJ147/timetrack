// use std::time::{Duration, SystemTime,UNIX_EPOCH};
// use std::thread::sleep;
mod session;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        println!("{arg}");
    }

    let foo = session::new_offline("example session".to_string());
    foo.start();
    foo.end();
}
