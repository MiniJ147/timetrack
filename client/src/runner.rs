
use crate::{evaluator, initializer, lexer, note, session};

struct Runner {
    conn: rusqlite::Connection,
    state: evaluator::State,
}

impl Runner { 
    pub fn session_start(&self) {
        let res = session::get_active(&self.conn);
        if let Some(s) = res {
            println!("resuming session {0}",s.name);
            session::update_active(&self.conn, true);
            return 
        }

        
        println!("creating session");
        match &self.state.value {
            Some(name) => session::create(&self.conn, &name),
            None => session::create(&self.conn, &String::from("new_session")),
        }
    }

    pub fn session_time(&self) {
        session::update_time_elapsed(&self.conn);
        let s = self.session_get_active();

        println!("session -> {0}: {1}",s.name, self.session_format_time(s.time_elapsed));
    }
    
    pub fn session_pause(&self) {
        println!("pausing session");
        session::update_active(&self.conn, false);
    }

    pub fn session_end(&self) {
        session::update_time_elapsed(&self.conn);
        let s = self.session_get_active();

        session::end(&self.conn);
        println!("ended session {0} with elapsed time: {1}",s.name, self.session_format_time(s.time_elapsed));
    }

    pub fn session_notes(&self) {
        let s = self.session_get_active();

        if let Some(msg) = &self.state.value {
            note::create(&self.conn, msg, s.id, session::ID);
            println!("successfully created message");
            return;
        }

        let notes = note::fetch_via_id(&self.conn, s.id);
        for note in notes.iter() {
            println!("{:?}",note);
        }
    }

    fn session_get_active(&self) -> session::Session {
        if let Some(session) = session::get_active(&self.conn) {
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
        "m" => runner.session_notes(),
        _ => panic!("invalid arg"),
    }
}
