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

pub struct Session {
    id: i32,
    name: String,
    time_elapsed: i64, 
    time_current: i64,
    time_ended: i64,
    active: i8
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
