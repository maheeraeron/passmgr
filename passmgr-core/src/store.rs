use crate::model::credential::Credential;
use chrono::Utc;
use rusqlite::Connection;
use rusqlite::Result;
use rusqlite::params;

/// Inserts a new credential into the database.
///
/// # Arguments
///
/// * `conn` - An active `SQLite` connection.
/// * `credential` - The `Credential` to store.
///
/// # Errors
///
/// Returns an error if the insert fails.
pub fn insert_credential(conn: &Connection, credential: &Credential) -> Result<()> {
    conn.execute(
        "
        INSERT INTO credentials (
            id, url, description, username, password, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
        ",
        params![
            credential.id.to_string(),
            credential.url,
            credential.description,
            credential.username,
            credential.password,
            credential.created_at.to_rfc3339(),
            credential.updated_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Fetches all stored credentials from the database.
///
/// # Arguments
///
/// * `conn` - An active `SQLite` connection.
///
/// # Errors
///
/// Returns an error if the query fails or if deserialization fails.
pub fn get_all_credentials(conn: &Connection) -> Result<Vec<Credential>> {
    let mut stmt = conn.prepare(
        "
        SELECT id, url, description, username, password, created_at, updated_at
        FROM credentials;
        ",
    )?;

    let results = stmt.query_map([], |row| {
        let id_str: String = row.get(0)?;
        let created_str: String = row.get(5)?;
        let updated_str: String = row.get(6)?;

        let id = uuid::Uuid::parse_str(&id_str).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&created_str)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    5,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        Ok(Credential {
            id,
            url: row.get(1)?,
            description: row.get(2)?,
            username: row.get(3)?,
            password: row.get(4)?,
            created_at,
            updated_at,
        })
    })?;

    results.collect()
}
