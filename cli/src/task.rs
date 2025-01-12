use crate::{error, evaluator};

mod runner;
mod db;

struct Runner {
    conn: rusqlite::Connection,
    state: evaluator::State,
}

const HELP_STR: &str = "
TASK HELP:
-h: [help] and list commands and functions.
-n: [new] creates new task given a value for the name.
";

pub fn run(conn: rusqlite::Connection, state: evaluator::State) {
    let runner = Runner { conn, state};

    match runner.state.args.as_str() {
        "n" => runner.create(),
        "a" => runner.active(),
        "h" => println!("{HELP_STR}"),
        _ => error::fail("TASK: invalid task argument please use -h for more information", 1),
    }
}
