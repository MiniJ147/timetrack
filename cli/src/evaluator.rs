use crate::initializer;
use crate::lexer::{Token,Type,Action};

#[derive(Debug)]
pub struct State {
    pub action: Action,
    pub args: String,
    pub value: Option<String>,
}

const SESSION_ARGS: [&str;16] = ["s","start","e","end","p","pause","v","view","m","message","t","time", "h", "help", "d", "delete"];
const TASK_ARGS: [&str;6] = ["h","help","n","new","a","active"]; // to be implemented 
const LIST_ARGS: [&str;2] = ["h","help"]; // to be implemented

pub fn evaluate(tokens: &Vec<Token>) -> State {
    //value, keyword
    let mut should_verify = true;
    let mut state = State{action: Action::None, args: "".to_string(), value: None};

    let expected = vec![Type::Value, Type::Keyword, Type::Arg, Type::Value];
    assert!(tokens.len() <= expected.len());

    for (i, token) in tokens.iter().enumerate(){
        if !token.kind.eq(&expected[i]) {
            eprintln!("EVALUATOR: unexpected token, recived: {0:?}, expected {1:?}", token.kind, expected[i]);
            std::process::exit(1);
        }
        
        match expected[i] {
            Type::Keyword => {
                state.action = token.action;

                // breaking early skips verfication which it should
                if state.action.eq(&Action::Help) || state.action.eq(&Action::Init) || state.action.eq(&Action::Drop) {
                    should_verify = false;
                    break; 
                }
            },
            Type::Arg => {
                let arg = token.str.replace("-","");

                if !validate_arg(state.action, &arg[..]){
                    eprintln!("EVALUATOR: {} is an invalid argument please use -h for more info",arg);
                    std::process::exit(1);
                }

                state.args = arg; 
            },
            Type::Value => {
                if i > 0 { // ensure we aren't copying the exe value
                    state.value = Some(token.str.clone())
                }
            }
        }
    }

    if should_verify { 
        if let Err(s) = initializer::verify() {
            eprintln!("{s}");
            std::process::exit(1);
        }
    }

    state 
}

fn validate_arg(action: Action, arg_trimed: &str) -> bool {
    match action {
        Action::Session => SESSION_ARGS.contains(&arg_trimed),
        Action::Task => TASK_ARGS.contains(&arg_trimed), 
        Action::List => LIST_ARGS.contains(&arg_trimed),
        _ => false, // covers help, init, None
    }
}
