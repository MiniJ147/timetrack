use crate::evaluator;

mod db;
mod runner;

struct Runner {
    conn: rusqlite::Connection,
    state: evaluator::State,
}

const HELP_STR: &str = "
SESSION HELP:
-h: help and list commands and functions. This can be called on each argument for more information.
-v: [view], list time, name, tasks, and notes about the session.
-s: [start] starts a new session and takes an optional name value. If a session already exists it will resume the session
-p: [pause] pauses the current session. Use -s to resume session.
-e: [end] ends active session. 
-d: [delete] deletes session based off session id given by the value.
-t: [time] gives time of session.
-m: [message] adds message if value is given, else it will list all messages attached to this session.
";

pub fn run(state: evaluator::State, conn: rusqlite::Connection) {
    let runner = Runner {
        conn: conn,
        state: state,
    };

    match runner.state.args.as_str(){
        "s" => runner.start(),
        "t" => runner.time(),
        "p" => runner.pause(),
        "e" => runner.end(),
        "m" => runner.notes(),
        "d" => runner.delete(),
        "h" => println!("{}",HELP_STR),
        _ => panic!("SESSION: invalid argument please run [session -h]"),
    }
}

