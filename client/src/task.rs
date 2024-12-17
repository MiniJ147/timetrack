use std::time::{Duration, SystemTime};

struct Note {
    timestamp: Duration, // task duration that it was created at
    message: String,
}

pub struct Task {
    name: String,
    time_elapsed: Duration,
    time_current: SystemTime,
    active: bool,
    completed: bool,
    notes: Vec<Note>,
}

impl Task {
    pub fn update_duration(&mut self, delta: Duration) {
        self.time_elapsed += delta
    }
    
    pub fn update_completed(&mut self, res: bool) {
        self.completed = res;
    }

    // print results to console
    pub fn view(&self){ 
        println!("Task: {0}\nDuration: {1}\nCompleted: {2}",self.name,self.time_elapsed.as_secs(),self.completed)
    }

    pub fn add_note(&mut self, message: String) {
        self.notes.push(Note{
            timestamp: self.time_elapsed, //auto clones?
            message,
        });
    }
}

pub fn new(name: String) -> Task {
    Task{
        name,
        time_elapsed: Duration::new(0, 0),
        time_current: SystemTime::now(),
        active: false,
        completed: false,
        notes: Vec::new(),
    }
}
