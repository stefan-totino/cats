///
/// This module acts the bridge between a sqlite database and 
/// higher level functions in the call stack
/// 
/// there is no ORM in use for the sake of this application, just a database driver
/// 
/// for a production application i would look to find a database driver which supports
/// async/await in order to not hold up clients on long running queries.
/// 
use crate::repository::cat_store::Cat;
use rusqlite::{Connection, Error as RusqliteError, Result};

#[derive(Debug)]
pub enum SqlStorageError {
    Rusqlite(RusqliteError),
}

pub struct SqlStorage {
    pub db_path: &'static str,
}

/// ideally we would want to take in a generic struct and update all of it's properties
/// in the database using an ORM, but for the sake of time and this app, i've implemented a function
/// where you can update only one column at a time, where the provided ID is the primary key.
pub fn update_column_by_id(
    table: &str,
    id: &str,
    col_key: &str,
    col_value: &str,
    conn: &mut Connection,
) -> Result<usize, SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::Rusqlite)?;
    let query = format!(
        "UPDATE {} SET {} = {:?} WHERE id = {}",
        table, col_key, col_value, id
    );
    let rows_updated = tx
        .execute(&query, [])
        .map_err(SqlStorageError::Rusqlite)?;
    tx.commit().map_err(SqlStorageError::Rusqlite)?;

    Ok(rows_updated)
}

pub fn delete_by_id(table: &str, id: &str, conn: &mut Connection) -> Result<(), SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::Rusqlite)?;
    let query = format!("DELETE FROM {} WHERE id = {}", table, id);
    tx.execute(&query, [])
        .map_err(SqlStorageError::Rusqlite)?;
    tx.commit().map_err(SqlStorageError::Rusqlite)?;

    Ok(())
}

/// this is the only SQL function i've written that will succeed if no database exists yet.
/// if one transaction fails, the previous in this scope will be rolled back.
pub fn insert_cat(name: &str, conn: &mut Connection) -> Result<String, SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::Rusqlite)?;

    tx.execute(
        "CREATE TABLE IF NOT EXISTS cats (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        [],
    )
    .map_err(SqlStorageError::Rusqlite)?;
    tx.execute("INSERT INTO cats (name) VALUES (?1)", [&name])
        .map_err(SqlStorageError::Rusqlite)?;

    let last_id = tx.last_insert_rowid().to_string();

    tx.commit().map_err(SqlStorageError::Rusqlite)?;

    Ok(last_id)
}

/// for a proper production ready app, we would want to implement paging here and reflect that 
/// notion higher in the call stack. Ideally the client would provide some sort of offset and limit in order 
/// to keep the size of this query reasonable.
/// 
/// here we simply return all cats in the sqlite database.
pub fn select_cats(conn: &mut Connection) -> Result<Vec<Cat>, SqlStorageError> {
    let mut statement = conn
        .prepare("SELECT * FROM cats")
        .map_err(SqlStorageError::Rusqlite)?;
    let rows = statement
        .query_map([], |row| {
            Ok(Cat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(SqlStorageError::Rusqlite)?;
    let mut cats = Vec::new();
    for cat in rows {
        cats.push(cat.map_err(SqlStorageError::Rusqlite)?);
    }
    Ok(cats)
}

pub fn select_cats_by_id(id: &str, conn: &mut Connection) -> Result<Vec<Cat>, SqlStorageError> {
    let mut statement = conn
        .prepare("SELECT id, name FROM cats WHERE id = (?1)")
        .map_err(SqlStorageError::Rusqlite)?;
    let rows = statement
        .query_map([id], |row| {
            Ok(Cat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(SqlStorageError::Rusqlite)?;
    let mut cats = Vec::new();
    for cat in rows {
        cats.push(cat.map_err(SqlStorageError::Rusqlite)?);
    }
    Ok(cats)
}
