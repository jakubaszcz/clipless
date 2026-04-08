use rusqlite::Connection;

pub(crate) fn init_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open("clipboard.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clips (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub(crate) fn insert_clip(conn: &Connection, content: &str) -> rusqlite::Result<()> {
    conn.execute("INSERT INTO clips (content) VALUES (?1)", [content])?;
    Ok(())
}