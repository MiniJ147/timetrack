use std::time::{Duration, SystemTime};


pub trait Session {
    fn start(&self);
    fn end(&self);
}

struct SessionOffline {
    name: String, 
    time_created: SystemTime, // start of session 
    time_current: SystemTime, // last known time since start or resume of session
    time_ended: SystemTime, // offical end of session 
    time_duration: Duration, // total duration not including pauses
    active: bool, // should we be tracking time
}

impl Session for SessionOffline{
    fn start(&self){
        println!("starting offline session");
    }

    fn end(&self){
        println!("ending offline session");
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
    })
}

