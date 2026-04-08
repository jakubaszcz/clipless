use std::collections::HashMap;
use std::process::id;
use rusqlite::{Connection, Map};
use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
fn app_path() -> PathBuf {
    let dirs = ProjectDirs::from("com", "jakubaszcz", "clipless").unwrap();

    let data_dir = dirs.data_dir();
    fs::create_dir_all(data_dir).unwrap();

    data_dir.join("clipless.db")
}
pub(crate) fn init_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open(app_path())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub(crate) fn remove_clip(conn: &Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM clips WHERE id = ?", [id])?;
    Ok(())
}
pub(crate) fn get_clips(conn: &Connection) -> rusqlite::Result<HashMap<u32, String>> {
    let mut stmt = conn.prepare("SELECT id, content FROM clips")?;
    let mut rows = stmt.query([])?;
    let mut clips = HashMap::new();
    while let Some(row) = rows.next()? {
        let id: u32 = row.get(0)?;
        let content: String = row.get(1)?;
        clips.insert(id, content);
    }
    Ok(clips)
}

pub(crate) fn insert_clip(conn: &Connection, content: &str) -> rusqlite::Result<()> {
    conn.execute("INSERT INTO clips (content) VALUES (?1)", [content])?;
    Ok(())
}