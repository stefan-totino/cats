use crate::repository::cat_store::Cat;
use rusqlite::{Connection, Error as RusqliteError, Result};

#[derive(Debug)]
pub enum SqlStorageError {
    RusqliteError(RusqliteError),
}

pub struct SqlStorage {
    pub db_path: &'static str,
}

pub fn update_column_by_id(
    table: &str,
    id: &str,
    col_key: &str,
    col_value: &str,
    conn: &mut Connection,
) -> Result<usize, SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::RusqliteError)?;
    let query = format!(
        "UPDATE {} SET {} = {:?} WHERE id = {}",
        table, col_key, col_value, id
    );
    let rows_updated = tx
        .execute(&query, [])
        .map_err(SqlStorageError::RusqliteError)?;
    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(rows_updated)
}

pub fn delete_by_id(table: &str, id: &str, conn: &mut Connection) -> Result<(), SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::RusqliteError)?;
    let query = format!("DELETE FROM {} WHERE id = {}", table, id);
    tx.execute(&query, [])
        .map_err(SqlStorageError::RusqliteError)?;
    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(())
}

pub fn insert_cat(name: &str, conn: &mut Connection) -> Result<String, SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::RusqliteError)?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS cats (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )
    .map_err(SqlStorageError::RusqliteError)?;
    tx.execute("INSERT INTO cats (name) VALUES (?1)", [&name])
        .map_err(SqlStorageError::RusqliteError)?;

    let last_id = tx.last_insert_rowid().to_string();

    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(last_id)
}

pub fn select_cats(conn: &mut Connection) -> Result<Vec<Cat>, SqlStorageError> {
    let mut statement = conn
        .prepare("SELECT * FROM cats")
        .map_err(SqlStorageError::RusqliteError)?;
    let rows = statement
        .query_map([], |row| {
            Ok(Cat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(SqlStorageError::RusqliteError)?;
    let mut cats = Vec::new();
    for cat in rows {
        cats.push(cat.map_err(SqlStorageError::RusqliteError)?);
    }
    Ok(cats)
}

pub fn select_cats_by_id(id: &str, conn: &mut Connection) -> Result<Vec<Cat>, SqlStorageError> {
    let mut statement = conn
        .prepare("SELECT id, name FROM cats WHERE id = (?1)")
        .map_err(SqlStorageError::RusqliteError)?;
    let rows = statement
        .query_map([id], |row| {
            Ok(Cat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(SqlStorageError::RusqliteError)?;
    let mut cats = Vec::new();
    for cat in rows {
        cats.push(cat.map_err(SqlStorageError::RusqliteError)?);
    }
    Ok(cats)
}
