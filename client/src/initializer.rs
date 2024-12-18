use rusqlite::Connection;

use crate::db; 

fn get_abs_path() -> String {
    std::env::var("HOME").expect("HOME enviroment variable not set")+"/.config/timetrack"
}

pub fn verify() -> Result<(),String>{ 
    let abs_path = get_abs_path(); 

    let exists = std::fs::exists(&abs_path).expect("failed to verify if ~/.config/timetrack/ exists");
    if !exists {
         return Err("~/.config/timetrack/ does not exists please run: timetrack init".to_string());
    }

    let exists_db = std::fs::exists(abs_path+"/data.db").expect("failed to verify if ~/.config/timetrack/data.db exists");
    if !exists_db{
        return Err("~/.config/timetrack/data.db does not exists please run: timetrack init".to_string());
    }

    Ok(())
}

pub fn init() {    
    let abs_path = get_abs_path();

    let exists = std::fs::exists(&abs_path).expect("failed to verify if ~/.config/timetrack/ exists");
    if !exists{
        std::fs::create_dir_all(&abs_path).unwrap();
    }

    let conn = Connection::open(abs_path+"/data.db").expect("failed to create data.db file with sqlite");
    conn.execute_batch(db::QUERY_INIT).expect("failed initilization query");
}

pub fn reset() {
    let abs_path = get_abs_path();
    std::fs::remove_file(abs_path+"/data.db").expect("failed database drop");
}
