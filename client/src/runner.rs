use crate::{db, evaluator, initializer, lexer};

pub fn run() {
    let tokens = lexer::parse_args();
    let state = evaluator::evaluate(&tokens);

    println!("{:?}",state);
    match state.action {
        lexer::Action::Session => run_session(&state.args, &state.value),
        lexer::Action::Help => println!("here is some help"),
        lexer::Action::Init => initializer::init(),
        lexer::Action::Task => println!("expect"),
        lexer::Action::List => println!("list"),
        lexer::Action::Drop => initializer::reset(),
        _ => panic!("invalid action"),
    }
}

fn run_session(arg: &str, value: &String) {
    match arg {
        "s" => print!("hello\n"),
        _ => panic!("invalid session argument"),
    }
}

pub fn session_create(conn: &rusqlite::Connection, name: String) {
    let res = db::session_get_active(conn);
    if let Some(s) = res {
        panic!("failed to create session, because session already exists! {0}",s.name);
    }

    db::session_create(conn, name);
}

