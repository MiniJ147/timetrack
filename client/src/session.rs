use core::panic;
use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Note {
    timestamp: Duration, // time in session note was taken
    message: String,
}

pub trait Session {
    fn start(&mut self);
    fn end(&mut self);
    fn pause(&mut self);
    fn duration(&mut self) -> Duration;
    fn add_note(&mut self, message: String);
}

/*
* time_current is our calculation time so it is important to keep it updated as it is the time
* since last action
*/

#[derive(Serialize,Deserialize)]
pub struct SessionOffline {
    name: String, 
    time_created: SystemTime, // start of session 
    time_current: SystemTime, // last known time since start or resume of session
    time_ended: SystemTime, // offical end of session 
    time_duration: Duration, // total duration not including pauses
    active: bool,// should we be tracking time
    ended: bool,
    notes: Vec<Note>
}

impl Session for SessionOffline{
    fn start(&mut self){
        if self.active {
            println!("DEBUG: session active thus it cannot start");
            return;
        }

        self.active = true;
        self.time_current = SystemTime::now();
        println!("starting session");
    }

    fn end(&mut self){
        if self.ended {
            println!("DEBUG: session already ended thus cannot end");
            return;
        }
        self.ended = true;
        self.active = false;
        self.time_ended = SystemTime::now();

        println!("ending offline session");
    }

    fn pause(&mut self) {
        if !self.active {
            println!("DEBUG: session not active thus it cannot puase");
            return;
        }

        self.time_update(); 
        self.active = false;
    }

    fn duration(&mut self) -> Duration{
        if self.active {
            self.time_update();
        }

        self.time_duration
    }

    fn add_note(&mut self, message: String) {
        let new_note = Note{
            timestamp: self.duration(),
            message,
        };
        
        self.notes.push(new_note);

        // for n in self.notes.iter() {
            // println!("{0},{1}",n.message,format_time(&n.time_created));
        // }
    }
}

impl SessionOffline {
    fn time_update(&mut self){
        // old impl 
        // let now = SystemTime::now();
        //
        // self.time_duration += now.duration_since(self.time_current).unwrap();
        // self.time_current = now;
        //
        self.time_duration += self.time_current.elapsed().expect("error when updating offline time");
        self.time_current = SystemTime::now();
    }
}

pub fn new_offline(name: String) -> Box<dyn Session>{
    let time_init = SystemTime::now();
    
    Box::new(SessionOffline { 
        name, 
        time_created: time_init, 
        time_current: time_init, 
        time_ended: time_init, 
        time_duration: Duration::new(0,0),
        active: false,
        ended: false,
        notes: Vec::new() 
    })
}

#[derive(Serialize,Deserialize)]
pub struct SessionOnline {}

impl Session for SessionOnline {
    fn start(&mut self){
        panic!("function not implemented!");
    }
    fn pause(&mut self){
        panic!("function not implemented!");
    }
    fn end(&mut self){
        panic!("function not implemented!");
    }
    fn duration(&mut self) -> Duration {
        panic!("function not implemented!");
    } 
    fn add_note(&mut self, _message: String) {
        panic!("function not implemented!");
    }
}


pub fn new_online() -> Box<dyn Session> {
    panic!("warning not implemented");
}

// utility 

pub fn format_time(duration: &Duration) -> String {
    let time_secs = duration.as_secs();
    let hours = time_secs/3600;
    let minutes = (time_secs/60) - (hours*60);
    let seconds = time_secs - ((hours*3600) + (minutes*60));
   
    let minute_str = if minutes < 10 {format!("0{minutes}")} else {format!("{minutes}")};
    let second_str = if seconds < 10 {format!("0{seconds}")} else {format!("{seconds}")};
    
    format!("{hours}:{minute_str}:{second_str}")
}
