use crate::{evaluator, initializer, lexer, session, task};


pub fn execute(){   
    let conn = initializer::create_connection();
    let state = evaluator::evaluate(&lexer::parse_args());

    match state.action {
        lexer::Action::Session => session::run(state, conn),
        lexer::Action::Help => println!("here is some help"),
        lexer::Action::Init => initializer::init(),
        lexer::Action::Task => task::run(conn, state), 
        lexer::Action::List => println!("list"),
        lexer::Action::Drop => initializer::reset(),
        _ => panic!("invalid action"),
    }
}
