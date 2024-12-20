use rusqlite::Connection;

use crate::db; 

fn get_abs_path() -> String {
    std::env::var("HOME").expect("HOME enviroment variable not set")+"/.timetrack.db"
}

pub fn verify() -> Result<(),String>{ 
    let abs_path = get_abs_path(); 

    let exists = std::fs::exists(&abs_path).expect("failed to verify if ~/.timetrack.db exists");
    if !exists{
        return Err("~/.timetrack.db does not exists please run: timetrack init".to_string());
    }

    Ok(())
}

pub fn init() {    
    let abs_path = get_abs_path();

    let conn = Connection::open(&abs_path).expect("failed to create data.db file with sqlite");
    conn.execute_batch(db::QUERY_INIT).expect("failed initilization query");
}

pub fn reset() {
    let abs_path = get_abs_path();
    std::fs::remove_file(abs_path).expect("failed database drop");
}
