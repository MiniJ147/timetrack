use crate::note;
use crate::session::db;

use super::Runner;

impl Runner { 
    pub fn start(&self) {
        let res = db::get_active(&self.conn);
        if let Some(s) = res {
            println!("resuming session {0}",s.name);
            db::update_active(&self.conn, true);
            return 
        }

        
        println!("creating session");
        match &self.state.value {
            Some(name) => db::create(&self.conn, &name),
            None => db::create(&self.conn, &String::from("new_session")),
        }
    }

    pub fn time(&self) {
        // db::update_time_elapsed(&self.conn);
        let s = self.get_active();

        println!("session -> {0}: {1}",s.name, self.format_time(s.time_elapsed));
    }
    
    pub fn pause(&self) {
        println!("pausing session");
        db::update_active(&self.conn, false);
    }

    pub fn end(&self) {
        // db::update_time_elapsed(&self.conn);
        let s = self.get_active();

        db::end(&self.conn);
        println!("ended session {0} with elapsed time: {1}",s.name, self.format_time(s.time_elapsed));
    }

    pub fn notes(&self) {
        let s = self.get_active();

        if let Some(msg) = &self.state.value {
            note::create(&self.conn,s.time_elapsed,msg, s.id, db::ID);
            println!("successfully created message");
            return;
        }

        let notes = note::fetch_via_id(&self.conn, s.id);
        for note in notes.iter() {
            println!("{:?}",note);
        }
    }
    
    pub fn delete(&self) {
        if let Some(id) = &self.state.value {
            let u32_id = id.parse::<u32>().unwrap();
            db::delete(&self.conn, u32_id);
            println!("successfully deleted {u32_id}");
        }else{
            eprintln!("must provide id: timetrack session -d [id]");
        } 
    }

    fn get_active(&self) -> db::Session {
        db::update_time_elapsed(&self.conn);
        if let Some(session) = db::get_active(&self.conn) {
            return session
        }
        
        eprintln!("no active session");
        std::process::exit(2);
    }

    fn format_time(&self, time_secs: u64) -> String {
        let time_secs = time_secs;
        let hours = time_secs/3600;
        let minutes = (time_secs/60) - (hours*60);
        let seconds = time_secs - ((hours*3600) + (minutes*60));
   
        let minute_str = if minutes < 10 {format!("0{minutes}")} else {format!("{minutes}")};
        let second_str = if seconds < 10 {format!("0{seconds}")} else {format!("{seconds}")};
    
        format!("{hours}:{minute_str}:{second_str}")
    }
}

