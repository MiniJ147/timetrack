use rusqlite::{Connection, params};

const QUERY_CREATE: &str = "
INSERT INTO tasks (name,time_current)
VALUES (?1, UNIXEPOCH()) 
";

// why did the sepreate these two from TRUE and FALSE istead of using case
// well we need to check when marking active for time managment updates like we do with sessions
// so instead of fetching the tasks to get tis
const QUERY_UPDATE_ACTIVE: &str = "
UPDATE tasks 
SET active = ?1
WHERE id = ?2
";

const QUERY_UPDATE_ELAPSED: &str = "
WITH time_save AS (SELECT UNIXEPOCH() as time_now)
UPDATE tasks
SET time_elapsed = time_elapsed + (time_now - time_current), time_current = time_now 
FROM time_save 
WHERE active = 1;
";

const QUERY_UPDATE_TIME: &str = "
UPDATE tasks 
SET time_current = UNIXEPOCH()
";

const QUERY_FIND: &str = "
SELECT * FROM tasks WHERE id = ?1 
";

#[derive(Debug)]
pub(crate) struct Task {
    pub id: u32,
    pub name: String,
    pub time_elapsed: u64,
    pub time_current: u64,
    pub active: i8,
    pub completed: i8 
}


pub(crate) fn create(name: &String, conn: &Connection) {
    conn.execute(QUERY_CREATE, params![name]).expect("DB: failed insertion on task");
}

// TODO: add checks to make sure changes are made through the rows effected given by conn.execute
pub(crate) fn update_active(value: bool, id: u32, conn: &Connection) {
    let value = match value {
        true => { update_time_current(conn); 1 },
        false => { update_time_elapsed(conn); 0 }
    }; 
 
    conn.execute(QUERY_UPDATE_ACTIVE, params![value, id]).expect("DB: failed update_active on task");
}

pub(crate) fn find(id: u32, conn: &Connection) -> Option<Task> {
    let mut stmt = conn.prepare(QUERY_FIND).unwrap();

    let result = stmt.query_row(params![id], |row| {
        Ok(Task{
            id: row.get(0)?,
            name: row.get(1)?,
            time_elapsed: row.get(2)?,
            time_current: row.get(3)?,
            active: row.get(4)?,
            completed: row.get(5)?
        })
    });

    match result {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

fn update_time_current(conn: &Connection) {
    conn.execute(QUERY_UPDATE_TIME, params![]).expect("failed to upadate time current for tasks");
}

fn update_time_elapsed(conn: &Connection) {
    conn.execute_batch(QUERY_UPDATE_ELAPSED).expect("failed to update elpased for task");
}
