
#[derive(PartialEq)]
pub enum Type {
    Keyword,
    Arg,
    Value,
}

#[derive(Clone, Copy, Debug)]
pub enum Action{
    Session,
    Task,
    List,
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
        _ => {
            if arg.contains("-") || arg.contains("--") {
                Token{kind:Type::Arg, str: arg.to_string(),action:Action::None}
            }else{
                Token{kind:Type::Value, str: arg.to_string(),action:Action::None}
            }
        }
    }
}
