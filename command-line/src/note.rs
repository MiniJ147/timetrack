use rusqlite::params;

const QUERY_CREATE: &str = "
INSERT INTO notes (timestamp, message, foreign_id, foreign_type)
VALUES (?1, ?2, ?3, ?4)
";

const QUERY_FETCH_VIA_ID: &str = "
SELECT * FROM notes WHERE foreign_id = ?1
";

const QUERY_FETCH_VIA_TYPE: &str = "
SELECT * FROM notes WHERE foreign_type = ?1
";

#[derive(Debug)]
pub struct Note {
    timestamp: u64,
    message: String,
    foreign_id: u32,
    foreign_type: u8
}

pub fn create(conn: &rusqlite::Connection, timestamp: u64, message: &String, foreign_id: u32, foreign_type: u8) {
    conn.execute(QUERY_CREATE,params![timestamp, message,foreign_id,foreign_type]).expect("failed to insert new note");
}

pub fn fetch_via_id(conn: &rusqlite::Connection, foreign_id: u32) -> Vec<Note> {
    let mut res: Vec<Note> = Vec::new();

    let mut stmt = conn.prepare(QUERY_FETCH_VIA_ID).unwrap();
    let mut rows = stmt.query(params![foreign_id]).unwrap();
    
    while let Some(row) = rows.next().unwrap() {
        res.push(Note{
            timestamp: row.get(0).unwrap(),
            message: row.get(1).unwrap(),
            foreign_id: row.get(2).unwrap(),
            foreign_type: row.get(3).unwrap(),
        })
    }

    res
}

pub fn fetch_via_type(conn: &rusqlite::Connection, foreign_type: u8) -> Vec<Note> {
    let mut res: Vec<Note> = Vec::new();

    let mut stmt = conn.prepare(QUERY_FETCH_VIA_ID).unwrap();
    let mut rows = stmt.query(params![foreign_type]).unwrap();
    
    while let Some(row) = rows.next().unwrap() {
        res.push(Note{
            timestamp: row.get(0).unwrap(),
            message: row.get(1).unwrap(),
            foreign_id: row.get(2).unwrap(),
            foreign_type: row.get(3).unwrap(),
        })
    }

    res
}

