use crate::repository::mock_store::MockStorage;
use crate::repository::sql_store::{self, SqlStorage, SqlStorageError};
use rusqlite::{Connection, Error as RusqliteError, Result};

#[derive(Debug)]
pub struct Cat {
    pub name: String,
}

#[derive(Debug)]
pub enum CatStorageError {
    MissingCat,
    NotUpdated,
    RusqliteError(RusqliteError),
    SqlStorageError(SqlStorageError),
}

pub trait CatStorage {
    fn new(db_path: &'static str) -> Self;
    fn create(&self, resource: Cat) -> Result<Cat, CatStorageError>;
    fn read(&self, resource_id: Option<String>) -> Result<Vec<Cat>, CatStorageError>;
    fn update(&self, resource_id: String, new_resource: Cat) -> Result<Cat, CatStorageError>;
    fn delete(&self, resource_id: String) -> Result<(), CatStorageError>;
}

impl CatStorage for SqlStorage {
    fn new(db_path: &'static str) -> SqlStorage {
        SqlStorage { db_path }
    }

    fn create(&self, cat: Cat) -> Result<Cat, CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
        let id = sql_store::insert_into_tx("cats", &cat.name, &mut conn)
            .map_err(CatStorageError::SqlStorageError)?;

        match self.read(Some(id))?.first() {
            Some(cat) => Ok(Cat {
                name: cat.name.clone(),
            }),
            None => Err(CatStorageError::MissingCat),
        }
    }

    fn read(&self, id: Option<String>) -> Result<Vec<Cat>, CatStorageError> {
        let column1 = match id {
            Some(value) => {
                let mut conn =
                    Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
                sql_store::select_col1_by_id("cats", &value, &mut conn)
                    .map_err(CatStorageError::SqlStorageError)?
            }
            None => {
                let mut conn =
                    Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
                sql_store::select_star("cats", &mut conn)
                    .map_err(CatStorageError::SqlStorageError)?
            }
        };

        Ok(column1
            .into_iter()
            .map(|cat| Cat { name: cat })
            .collect::<Vec<Cat>>())
    }

    fn update(&self, id: String, cat: Cat) -> Result<Cat, CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
        let update_count =
            sql_store::update_column_by_id_tx("cats", &id, "name", &cat.name, &mut conn)
                .map_err(CatStorageError::SqlStorageError)?;

        if update_count == 1 {
            match self.read(Some(id))?.first() {
                Some(cat) => Ok(Cat {
                    name: cat.name.clone(),
                }),
                None => Err(CatStorageError::MissingCat),
            }
        } else {
            Err(CatStorageError::NotUpdated)
        }
    }

    fn delete(&self, id: String) -> Result<(), CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
        sql_store::delete_by_id_tx("cats", &id, &mut conn)
            .map_err(CatStorageError::SqlStorageError)?;
        Ok(())
    }
}

impl CatStorage for MockStorage {
    fn new(_db_path: &'static str) -> MockStorage {
        MockStorage {}
    }

    fn create(&self, _cat: Cat) -> Result<Cat, CatStorageError> {
        unimplemented!()
    }
    fn read(&self, _id: Option<String>) -> Result<Vec<Cat>, CatStorageError> {
        unimplemented!()
    }
    fn update(&self, _id: String, _cat: Cat) -> Result<Cat, CatStorageError> {
        unimplemented!()
    }
    fn delete(&self, _id: String) -> Result<(), CatStorageError> {
        unimplemented!()
    }
}
