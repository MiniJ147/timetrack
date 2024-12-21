use crate::db;

pub fn session_create(conn: &rusqlite::Connection, name: String) {
    let res = db::session_get_active(conn);
    if let Some(s) = res {
        panic!("failed to create session, because session already exists! {0}",s.name);
    }

    db::session_create(conn, name);
}

