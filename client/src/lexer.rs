pub enum Token {
    Keyword(String),
    Arg(String),
    Value(String)
}

pub const KEYWORD_SESSION: &str = "session";
pub const KEYWORD_TASK: &str = "task";
pub const KEYWORD_LIST: &str = "list";

pub fn parse_args() ->Vec<Token>{
    let mut result: Vec<Token> = Vec::new();
    let args = std::env::args();

    for arg in args {
        result.push(generate_token(&arg));
    }

    result
}

fn generate_token(arg: &str) -> Token {
    match arg {
        "session" => Token::Keyword(KEYWORD_SESSION.to_string()),
        "s" => Token::Keyword(KEYWORD_SESSION.to_string()),
        "task" => Token::Keyword(KEYWORD_TASK.to_string()),
        "t" => Token::Keyword(KEYWORD_TASK.to_string()),
        _ => {
            if arg.contains("-") || arg.contains("--") {
                Token::Arg(arg.to_string()) 
            }else{
                Token::Value(arg.to_string())
            }
        }
    }
}
