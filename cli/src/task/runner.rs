use crate::error;

use super::{Runner,db};

//TODO: 
//FIX SHIT CODE YOU MADE TODAY AT 4am
//problem spaghetti mess when integrating both task and sessions 
//need clean managment of both task time with session time (which we might have done)
//we also need to make sure we can't active task when no session is active but currently that
//function is private so think of something better for these implementations to communicate
//also comment this bitch and make it readable and not confusing 
//TLDR: somewhat working just fix the spaghetti and bugs

impl Runner {
    pub fn create(&self) {
        match &self.state.value {
            Some(name) => {
                db::create(name, &self.conn);
                println!("successfully created task: {name}");
            }
            None => error::fail("must provide value for task name", 1), 
        }
    }

    pub fn active(&self) {
        if let Some(id) = &self.state.value {
            let id = id.parse::<u32>().expect("failed to parse id to int make sure it is valid number");
            let task = db::find(id,&self.conn).expect("failed to find task");

            // flips active 
            db::update_active(task.active == 0, id, &self.conn);
            println!("task is active = {}",task.active == 0);
        }else{
            error::fail("must provide id for task",1);
        }
    }
}
