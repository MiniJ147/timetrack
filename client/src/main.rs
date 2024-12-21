use client::{db, initializer, lexer, runner};
use rusqlite::Connection;

// use std::io;
fn main(){
    let tokens = lexer::parse_args();
    for token in tokens {
        match token {
            lexer::Token::Keyword(s) => println!("Keyword: {s}"),
            lexer::Token::Arg(s) => println!("Arg: {s}"),
            lexer::Token::Value(s) => println!("Value: {s}")
        }
    }
    // let args: Vec<String> = std::env::args().collect();
    // for arg in args {
    //     if arg=="init"{
    //         initializer::init();
    //     }
    //     if arg=="reset"{
    //         initializer::reset();
    //     }
    //     println!("{arg}");
    // }
    //
    // if let Err(s) = initializer::verify() {
    //     println!("{s}");
    //     std::process::exit(1);
    // }
    //
    // let path = std::env::var("HOME").expect("HOME enviroment variable not set")+"/.timetrack.db";
    // let conn = Connection::open(&path).expect("failed db connection");    
    //
    // // db::session_update_active(&conn, false);
    // db::session_update_time_elapsed(&conn);
    //
    // // db::session_end(&conn);
    //
    // let session = match db::session_get_active(&conn) {
    //     Some(s) => s,
    //     None => {
    //         println!("no session found!");
    //         db::session_create(&conn, "some new session".to_string());
    //         return;
    //     }
    // };
    //
    // println!("{:?}",session);
}
