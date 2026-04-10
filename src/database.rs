
use rusqlite::Connection;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use crate::clipboard;

// Get the path to the database file
fn app_path() -> PathBuf {
    let dirs = ProjectDirs::from("com",
                                 "jakubaszcz",
                                 "clipless").unwrap();

    let data_dir = dirs.data_dir();
    fs::create_dir_all(data_dir).unwrap();

    data_dir.join("clipless.db")
}

// Initialize the database
pub(crate) fn init_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open(app_path())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            timestamp INTEGER
        )",
        [],
    )?;

    Ok(conn)
}

// Remove a clip from the database
pub(crate) fn remove_clip(conn: &Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM clips WHERE id = ?", [id])?;
    Ok(())
}

// Get all clips from the database
pub(crate) fn get_clips(conn: &Connection) -> rusqlite::Result<Vec<clipboard::Clipboard>> {
    let mut stmt = conn.prepare("SELECT id, content, timestamp FROM clips")?;
    let mut rows = stmt.query([])?;
    let mut clips = Vec::new();
    while let Some(row) = rows.next()? {
        let clip = clipboard::Clipboard {
            id: row.get(0)?,
            content: row.get(1)?,
            timestamp: row.get(2)?,
        };
        clips.push(clip);
    }
    Ok(clips)
}

// Fetch clips from the database based on a search query
pub(crate) fn fetch_clips(conn: &Connection, query: &str) -> rusqlite::Result<Vec<clipboard::Clipboard>> {
    let mut stmt = conn.prepare("SELECT id, content, timestamp FROM clips WHERE content LIKE ?1 ESCAPE '\\'")?;
    let mut rows = stmt.query([format!("%{}%", query)])?;
    let mut clips = Vec::new();
    while let Some(row) = rows.next()? {
        let clip = clipboard::Clipboard {
            id: row.get(0)?,
            content: row.get(1)?,
            timestamp: row.get(2)?,
        };
        clips.push(clip);
    }
    Ok(clips)
}

// Insert a new clip into the database
pub(crate) fn insert_clip(conn: &Connection, content: &str) -> rusqlite::Result<()> {
    let now = Utc::now().timestamp();

    conn.execute(
        "INSERT INTO clips (content, timestamp) VALUES (?1, ?2)",
        (content, now),
    )?;

    Ok(())
}