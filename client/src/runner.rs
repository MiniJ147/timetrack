
use crate::{db, evaluator, initializer, lexer};

struct Runner {
    conn: rusqlite::Connection,
    state: evaluator::State,
}

impl Runner { 
    pub fn session_start(&self) {
        let res = db::session_get_active(&self.conn);
        if let Some(s) = res {
            println!("resuming session {0}",s.name);
            db::session_update_active(&self.conn, true);
            return 
        }

        println!("creating session");
        match &self.state.value {
            Some(name) => db::session_create(&self.conn, &name),
            None => db::session_create(&self.conn, &String::from("new_session")),
        }
    }

    pub fn session_time(&self) {
        db::session_update_time_elapsed(&self.conn);
        let s = self.session_get_active();

        println!("session -> {0}: {1}",s.name, self.session_format_time(s.time_elapsed));
    }
    
    pub fn session_pause(&self) {
        println!("pausing session");
        db::session_update_active(&self.conn, false);
    }

    pub fn session_end(&self) {
        db::session_update_time_elapsed(&self.conn);
        let s = self.session_get_active();

        db::session_end(&self.conn);
        println!("ended session {0} with elapsed time: {1}",s.name, self.session_format_time(s.time_elapsed));
    }

    fn session_get_active(&self) -> db::Session {
        if let Some(session) = db::session_get_active(&self.conn) {
            return session
        }
        
        eprintln!("no active session");
        std::process::exit(2);
    }

    fn session_format_time(&self, time_secs: i64) -> String {
        let time_secs = time_secs;
        let hours = time_secs/3600;
        let minutes = (time_secs/60) - (hours*60);
        let seconds = time_secs - ((hours*3600) + (minutes*60));
   
        let minute_str = if minutes < 10 {format!("0{minutes}")} else {format!("{minutes}")};
        let second_str = if seconds < 10 {format!("0{seconds}")} else {format!("{seconds}")};
    
        format!("{hours}:{minute_str}:{second_str}")
    }
}

pub fn run() {
    let runner = Runner{
        conn: initializer::create_connection(),
        state: evaluator::evaluate(&lexer::parse_args()), 
    };

    println!("{:?}",runner.state);
    match runner.state.action {
        lexer::Action::Session => run_session(&runner),
        lexer::Action::Help => println!("here is some help"),
        lexer::Action::Init => initializer::init(),
        lexer::Action::Task => println!("expect"),
        lexer::Action::List => println!("list"),
        lexer::Action::Drop => initializer::reset(),
        _ => panic!("invalid action"),
    }
}

fn run_session(runner: &Runner) {
    match runner.state.args.as_str(){
        "s" => runner.session_start(),
        "t" => runner.session_time(), 
        "p" => runner.session_pause(),
        "e" => runner.session_end(),
        _ => panic!("invalid arg"),
    }
}
