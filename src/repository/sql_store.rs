use rusqlite::{Connection, Error as RusqliteError, Result};

#[derive(Debug)]
pub enum SqlStorageError {
    Transaction,
    RusqliteError(RusqliteError),
}

pub struct SqlStorage {
    pub db_path: &'static str,
}

pub fn insert_into_tx(
    table: &str,
    value1: &str,
    conn: &mut Connection,
) -> Result<String, SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::RusqliteError)?;
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (id integer PRIMARY KEY, name text NOT NULL)",
        table
    );
    let query2 = format!("INSERT INTO {} (name) VALUES ({:?})", table, value1);
    tx.execute(&query, [])
        .map_err(SqlStorageError::RusqliteError)?;
    tx.execute(&query2, [])
        .map_err(SqlStorageError::RusqliteError)?;
    let last_id = tx.last_insert_rowid().to_string();

    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(last_id)
}

pub fn select_col1_by_id<T: rusqlite::types::FromSql>(
    table: &str,
    id: &str,
    conn: &mut Connection,
) -> Result<Vec<T>, SqlStorageError> {
    let query = format!("SELECT * FROM {} WHERE id = {}", table, id);
    let mut statement = conn
        .prepare(&query)
        .map_err(SqlStorageError::RusqliteError)?;
    let mut rows = statement
        .query([])
        .map_err(SqlStorageError::RusqliteError)?;

    let mut col1 = Vec::new();
    while let Some(row) = rows.next().map_err(SqlStorageError::RusqliteError)? {
        col1.push(row.get(1).map_err(SqlStorageError::RusqliteError)?);
    }

    Ok(col1)
}

pub fn select_star(table: &str, conn: &mut Connection) -> Result<Vec<String>, SqlStorageError> {
    let query = format!("SELECT * FROM {}", table);
    let mut statement = conn
        .prepare(&query)
        .map_err(SqlStorageError::RusqliteError)?;
    let mut rows = statement
        .query([])
        .map_err(SqlStorageError::RusqliteError)?;
    let mut names = Vec::new();
    while let Some(row) = rows.next().map_err(SqlStorageError::RusqliteError)? {
        names.push(row.get(1).map_err(SqlStorageError::RusqliteError)?);
    }

    Ok(names)
}

pub fn update_column_by_id_tx(
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
    println!("{:?}", query);
    let rows_updated = tx
        .execute(&query, [])
        .map_err(SqlStorageError::RusqliteError)?;
    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(rows_updated)
}

pub fn delete_by_id_tx(
    table: &str,
    id: &str,
    conn: &mut Connection,
) -> Result<(), SqlStorageError> {
    let tx = conn.transaction().map_err(SqlStorageError::RusqliteError)?;
    let query = format!("DELETE FROM {} WHERE id = {}", table, id);
    tx.execute(&query, [])
        .map_err(SqlStorageError::RusqliteError)?;
    tx.commit().map_err(SqlStorageError::RusqliteError)?;

    Ok(())
}
