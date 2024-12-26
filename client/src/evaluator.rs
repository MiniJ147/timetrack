use crate::lexer::{Token,Type,Action};

#[derive(Debug)]
pub struct State {
    action: Action,
    args: String,
    value: String,
}

const SESSION_ARGS: [&str;12] = ["s","start","e","end","p","pause","v","view","m","message","t","time"];
const TASK_ARGS: [&str;0] = []; // to be implemented 
const LIST_ARGS: [&str;0] = []; // to be implemented

pub fn evaluate(tokens: &Vec<Token>) {
    //value, keyword
    let mut state = State{action: Action::None, args: "".to_string(), value: "".to_string()}; 

    let expected = vec![Type::Value, Type::Keyword, Type::Arg, Type::Value];
    assert!(tokens.len() <= expected.len());

    for (i, token) in tokens.iter().enumerate(){
        if !token.kind.eq(&expected[i]) {
            println!("invalid");
            continue;
        }
        
        match expected[i] {
            Type::Keyword => state.action = token.action,
            Type::Arg => {
                let arg = token.str.replace("-","");

                assert!(validate_arg(state.action, &arg[..]));
                state.args = arg; 
            },
            Type::Value => state.value = token.str.clone(), 
        }
    }

    println!("{:?}",state);
}

fn validate_arg(action: Action, arg_trimed: &str) -> bool {
    match action {
        Action::Session => SESSION_ARGS.contains(&arg_trimed),
        Action::Task => TASK_ARGS.contains(&arg_trimed), 
        Action::List => LIST_ARGS.contains(&arg_trimed),
        Action::None => false,
    }
}
