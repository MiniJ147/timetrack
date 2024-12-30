use rusqlite::{Connection,params};


const QUERY_CREATE: &str = "
INSERT INTO task (name,time_current)
VALUES (?1, UNIXEPOCH()) 
";

pub fn create(conn: &Connection, name: &String) {
    conn.execute(QUERY_CREATE, params![name]).unwrap();
}
