#[derive(PartialEq, Debug)]
pub enum Type {
    Keyword,
    Arg,
    Value,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action{
    Session,
    Task,
    List,
    Init,
    Help,
    Drop,
    None 
}

pub struct Token {
    pub kind: Type,
    pub str: String,
    pub action: Action, 
}

pub const KEYWORD_SESSION: &str = "session";
pub const KEYWORD_TASK: &str = "task";
pub const KEYWORD_LIST: &str = "list";
pub const KEYWORD_INIT: &str = "init";
pub const KEYWORD_HELP: &str = "help";
pub const KEYWORD_DROP: &str = "drop";

pub fn parse_args() ->Vec<Token>{
    let mut result: Vec<Token> = Vec::new();
    let args = std::env::args();

    for arg in args {
        result.push(generate_token(&arg.trim()));
    }

    result
}

fn generate_token(arg: &str) -> Token {
    match arg {
        "session" => Token{kind:Type::Keyword, str: KEYWORD_SESSION.to_string(),action:Action::Session},
        "s" => Token{kind:Type::Keyword, str: KEYWORD_SESSION.to_string(), action:Action::Session},
        "task" => Token{kind:Type::Keyword, str:KEYWORD_TASK.to_string(), action:Action::Task},
        "t" => Token{kind:Type::Keyword, str: KEYWORD_TASK.to_string(), action:Action::Task},
        "help" => Token{kind:Type::Keyword, str: KEYWORD_HELP.to_string(), action:Action::Help},
        "init" => Token{kind:Type::Keyword, str: KEYWORD_INIT.to_string(), action:Action::Init},
        "drop" => Token{kind:Type::Keyword, str: KEYWORD_DROP.to_string(), action:Action::Drop},
        _ => {
            if arg.starts_with("-"){ 
                Token{kind:Type::Arg, str: arg.to_string(),action:Action::None}
            }else{
                Token{kind:Type::Value, str: arg.to_string(),action:Action::None}
            }
        }
    }
}
