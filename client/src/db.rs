use crate::error::Result;
use rusqlite::{params, Connection, OptionalExtension};

pub const QUERY_INIT: &str = " 
CREATE TABLE sessions (
  id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
  name text NOT NULL,
  time_elapsed bigint default 0 NOT NULL,
  time_current bigint NOT NULL,
  time_ended bigint default 0 NOT NULL,
  active int default 0 NOT NULL
);

CREATE TABLE tasks (
  id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
  name text NOT NULL,
  time_elapsed bigint DEFAULT 0 NOT NULL,
  time_current bigint NOT NULL,
  active int default 0 NOT NULL,
  completed int default 0 NOT NULL
);

CREATE TABLE notes(
  timestamp bigint NOT NULL,
  message text NOT NULL,
  foreign_id int NOT NULL,
  foreign_type int CHECK (foreign_type IN (0,1)) NOT NULL 
);
";

const QUERY_SESSION_FIND_ACTIVE: &str = "
SELECT * FROM sessions WHERE time_ended = 0
";

const QUERY_SESSION_INSERT: &str = "
INSERT INTO sessions (name,time_current,active) VALUES (?1, UNIXEPOCH(), 1)
";

const QUERY_SESSION_UPDATE_ELAPSED: &str = "
WITH time_save AS (SELECT UNIXEPOCH() as time_now)
UPDATE sessions
SET time_elapsed = time_elapsed + (time_now - time_current), time_current = time_now 
FROM time_save
WHERE active = 1;
";

const QUERY_SESSION_UPDATE_TIME: &str = "
UPDATE sessions
SET time_current = UNIXEPOCH()
WHERE time_ended = 0
";

const QUERY_SESSION_UPDATE_ACTIVE: &str = "
UPDATE sessions
SET active = ?1
WHERE time_ended = 0;
";

const QUERY_SESSION_END: &str = "
UPDATE sessions
SET active = 0, time_ended = UNIXEPOCH()
WHERE time_ended = 0
";

#[derive(Debug)]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub time_elapsed: i64, 
    pub time_current: i64,
    pub time_ended: i64,
    pub active: i8
}

pub struct Task {
    id: i32,
    name: String,
    time_elapsed: i64,
    time_current: i64,
    active: i8,
    completed: i8 

}
pub struct Note {
    timestamp: u64,
    message: String,
    foreign_id: i8,
    foreign_type: i8
}

/// returns the active session in Option
/// panics if SELECT query fails
pub fn session_get_active(conn: &Connection) -> Option<Session> { 
    let mut stmt = conn.prepare(QUERY_SESSION_FIND_ACTIVE).unwrap();

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
pub fn session_update_time_elapsed(conn: &Connection) {
    conn.execute_batch(QUERY_SESSION_UPDATE_ELAPSED).unwrap();
}

/// creates session does not check if a session already exists 
/// panics if query failed execution.
pub fn session_create(conn: &Connection, name: String) {
    conn.execute(QUERY_SESSION_INSERT, params![name]).expect("failed session insertion query");
}

/// updates current time on active session.
/// panics if execution fails.
pub fn session_update_time_current(conn: &Connection) {
    conn.execute(QUERY_SESSION_UPDATE_TIME,params![]).unwrap();
}

// updates current time.
// ends session by setting time_ended = time_now.
// panics if query fails or session_update_time_elapsed() fails.
pub fn session_end(conn: &Connection) {
    session_update_time_elapsed(conn);

    conn.execute(QUERY_SESSION_END, params![]).unwrap();
}   

/// updates session active status.
/// if true - calls session_update_time_current() to keep state.
/// if false - calls session_update_time_elapsed() to keep state.
/// fails if either functions fail or query fails.
pub fn session_update_active(conn: &Connection, value: bool) {
    let new_value = match value {
        true => {
            session_update_time_current(conn); 1
        },
        false => {
            session_update_time_elapsed(conn); 0
        }
    };

    conn.execute(QUERY_SESSION_UPDATE_ACTIVE, params![new_value]).unwrap();
}
