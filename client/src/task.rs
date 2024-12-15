use std::time::Duration;

struct Note {
    timestamp: Duration,
    session_name: String,
    message: String,
}

pub struct Task {
    name: String,
    time_elapsed: Duration,
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

    pub fn add_note(&mut self, session_name: String, message: String) {
        self.notes.push(Note{
            timestamp: self.time_elapsed, //auto clones?
            session_name,
            message,
        });
    }
}

pub fn new(name: String) -> Task {
    Task{
        name,
        time_elapsed: Duration::new(0, 0),
        completed: false,
        notes: Vec::new(),
    }
}
