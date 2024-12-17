use std::io;
mod session;
mod task;

fn main() {
    println!("Hello, world! To leave type: end");
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        println!("{arg}");
    }
    let mut session = session::new_offline("offline session 1".to_string());
    let mut task = task::new("example".to_string());

    task.add_note("hello task from this".to_string());
    task.view();

    loop {
        let mut input = String::new();
        print!(">> ");
        let _ = io::stdin().read_line(&mut input); 
        match input.trim() {
            "start" => session.start(),
            "pause" => session.pause(),
            "time" => println!("{}",session::format_time(&session.duration())),
            "add" => session.add_note("example message".to_string()),
            "end" => { 
                session.end(); 
                break;
            },
            _ => println!("invalid input"),
        }

    }

}
