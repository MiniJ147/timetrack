use crate::lexer;


pub fn evaluate(tokens: &Vec<lexer::Token>) {
    //value, keyword
    let expected = lexer::Type::Value;
    for token in tokens.iter() {
        if token.kind.eq(&expected) {
            println!("valued");
        }else{
            println!("invalid");
        }
    }
}

