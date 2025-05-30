use rusqlite::{params, Connection};

const QUERY_FIND_ACTIVE: &str = "
SELECT * FROM sessions WHERE time_ended = 0
";

const QUERY_INSERT: &str = "
INSERT INTO sessions (name,time_current,active) VALUES (?1, UNIXEPOCH(), 1)
";

const QUERY_UPDATE_ELAPSED: &str = "
WITH time_save AS (SELECT UNIXEPOCH() as time_now)
UPDATE sessions
SET time_elapsed = time_elapsed + (time_now - time_current), time_current = time_now 
FROM time_save
WHERE active = 1;
";

const QUERY_UPDATE_TIME: &str = "
UPDATE sessions
SET time_current = UNIXEPOCH()
WHERE time_ended = 0
";

const QUERY_UPDATE_ACTIVE: &str = "
UPDATE sessions
SET active = ?1
WHERE time_ended = 0;
";

const QUERY_END: &str = "
UPDATE sessions
SET active = 0, time_ended = UNIXEPOCH()
WHERE time_ended = 0
";

const QUERY_DELETE: &str = "
DELETE FROM sessions
WHERE id = ?1
";

pub const HELP_STR: &str = "
SESSION HELP:
-h: help and list commands and functions. This can be called on each argument for more information.
-v: [view], list time, name, tasks, and notes about the session.
-s: [start] starts a new session and takes an optional name value. If a session already exists it will resume the session
-p: [pause] pauses the current session. Use -s to resume session.
-e: [end] ends active session. 
-d: [delete] deletes session based off session id given by the value.
-t: [time] gives time of session.
-m: [message] adds message if value is given, else it will list all messages attached to this session.
";

/// id used inside database queries
pub const ID: u8 = 0;

#[derive(Debug)]
pub struct Session {
    pub id: u32,
    pub name: String,
    pub time_elapsed: u64, 
    pub time_current: u64,
    pub time_ended: u64,
    pub active: i8
}



pub fn get_active(conn: &Connection) -> Option<Session> { 
    let mut stmt = conn.prepare(QUERY_FIND_ACTIVE).unwrap();

    let result = stmt.query_row([], |row| {
        Ok(Session{
            id: row.get(0)?,
            name: row.get(1)?,
            time_elapsed: row.get(2)?,
            time_current: row.get(3)?,
            time_ended: row.get(4)?,
            active: row.get(5)?
        })
    });

    match result {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

/// updates elapsed time if open session exists and if it is active.
/// panics if query failed execution.
pub fn update_time_elapsed(conn: &Connection) {
    conn.execute_batch(QUERY_UPDATE_ELAPSED).unwrap();
}

/// creates session does not check if a session already exists 
/// panics if query failed execution.
pub fn create(conn: &Connection, name: &String) {
    conn.execute(QUERY_INSERT, params![name]).expect("failed session insertion query");
}

/// updates current time on active session.
/// panics if execution fails.
fn update_time_current(conn: &Connection) {
    conn.execute(QUERY_UPDATE_TIME,params![]).unwrap();
}

/// ends session by setting time_ended = time_now.
/// panics if query fails 
pub fn end(conn: &Connection) {
    conn.execute(QUERY_END, params![]).unwrap();
}   
/// deletes a session [DOES NOT DELETE NOTES]
/// panics if query fails
pub fn delete(conn: &Connection, id: u32) {
    conn.execute(QUERY_DELETE, params![id]).unwrap();
}

/// updates session active status.
/// value = true means resuming session 
/// value = false means pausing session
/// if true - calls session_update_time_current() to keep state.
/// if false - calls session_update_time_elapsed() to keep state.
/// fails if either functions fail or query fails.
pub fn update_active(conn: &Connection, value: bool) {
    let new_value = match value {
        true => {
            update_time_current(conn); 1
        },
        false => {
            update_time_elapsed(conn); 0
        }
    };

    conn.execute(QUERY_UPDATE_ACTIVE, params![new_value]).unwrap();
}
