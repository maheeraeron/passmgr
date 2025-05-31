use rusqlite::Connection;
use rusqlite::Result;

/// Initializes the database by creating the `credentials` table if it doesn't exist.
///
/// # Arguments
///
/// * `conn` - A valid `SQLite` connection object
///
/// # Returns
///
/// Returns `Ok(())` if the schema is successfully created.
///
/// # Errors
///
/// Returns a [`rusqlite::Error`] if executing the schema creation fails.
///
/// # Examples
///
/// ```rust
/// let conn = rusqlite::Connection::open_in_memory().unwrap();
/// passmgr_core::db::init_db(&conn).unwrap();
/// ```
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS credentials (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            description TEXT,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
