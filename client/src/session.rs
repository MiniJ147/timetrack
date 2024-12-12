use std::time::{Duration, SystemTime};


pub trait Session {
    fn start(&mut self);
    fn end(&mut self);
    fn pause(&mut self);
    fn time_get_total(&mut self) -> Duration;
}


/*
* time_current is our calculation time so it is important to keep it updated as it is the time
* since last action
*/
struct SessionOffline {
    name: String, 
    time_created: SystemTime, // start of session 
    time_current: SystemTime, // last known time since start or resume of session
    time_ended: SystemTime, // offical end of session 
    time_duration: Duration, // total duration not including pauses
    active: bool, // should we be tracking time
    ended: bool
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

    fn time_get_total(&mut self) -> Duration{
        if self.active {
            self.time_update();
        }

        self.time_duration
    }
}

impl SessionOffline {
    fn time_update(&mut self){
        let now = SystemTime::now();

        self.time_duration += now.duration_since(self.time_current).unwrap();
        self.time_current = now;
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
    })
}

