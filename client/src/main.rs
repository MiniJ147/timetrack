use std::io;
mod session;

fn main() {
    println!("Hello, world! To leave type: End");
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        println!("{arg}");
    }
    let mut session = session::new_offline("offline session 1".to_string());

    loop {
        let mut input = String::new();
        print!(">> ");
        let _ = io::stdin().read_line(&mut input); 
        match input.trim() {
            "Start" => session.start(),
            "Pause" => session.pause(),
            "Time" => println!("{} seconds",session.time_get_total().as_secs()),
            "End" => { 
                session.end(); 
                break;
            },
            _ => println!("invalid input"),
        }

    }

}
