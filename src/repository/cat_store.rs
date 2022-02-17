use crate::repository::sql_store::{self, SqlStorage, SqlStorageError};
use rusqlite::{Connection, Error as RusqliteError, Result};

#[derive(Debug)]
pub struct Cat {
    pub id: i32,
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
    fn create(&self, name: String) -> Result<Cat, CatStorageError>;
    fn read(&self, resource_id: Option<String>) -> Result<Vec<Cat>, CatStorageError>;
    fn update(&self, resource_id: String, new_resource: Cat) -> Result<Cat, CatStorageError>;
    fn delete(&self, resource_id: String) -> Result<(), CatStorageError>;
}

impl CatStorage for SqlStorage {
    fn new(db_path: &'static str) -> SqlStorage {
        SqlStorage { db_path }
    }

    fn create(&self, name: String) -> Result<Cat, CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
        let id = sql_store::insert_cat(&name, &mut conn)
            .map_err(CatStorageError::SqlStorageError)?;
        match self.read(Some(id))?.first() {
            Some(cat) => Ok(Cat {
                id: cat.id.clone(),
                name: cat.name.clone(),
            }),
            None => Err(CatStorageError::MissingCat),
        }
    }

    fn read(&self, id_opt: Option<String>) -> Result<Vec<Cat>, CatStorageError> {
        let cats = match id_opt {
            Some(id) => {
                let mut conn =
                    Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
                sql_store::select_cats_by_id(&id, &mut conn)
                    .map_err(CatStorageError::SqlStorageError)?
            }
            None => {
                let mut conn =
                    Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
                sql_store::select_cats(&mut conn).map_err(CatStorageError::SqlStorageError)?
            }
        };

        Ok(cats
            .into_iter()
            .map(|cat| Cat {
                id: cat.id,
                name: cat.name,
            })
            .collect::<Vec<Cat>>())
    }

    fn update(&self, id: String, cat: Cat) -> Result<Cat, CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::RusqliteError)?;
        let update_count =
            sql_store::update_column_by_id("cats", &id, "name", &cat.name, &mut conn)
                .map_err(CatStorageError::SqlStorageError)?;

        if update_count == 1 {
            match self.read(Some(id))?.first() {
                Some(cat) => Ok(Cat {
                    id: cat.id.clone(),
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
        sql_store::delete_by_id("cats", &id, &mut conn)
            .map_err(CatStorageError::SqlStorageError)?;
        Ok(())
    }
}

pub struct MockStorage {}

impl CatStorage for MockStorage {
    fn new(_db_path: &'static str) -> MockStorage {
        MockStorage {}
    }

    fn create(&self, _name: String) -> Result<Cat, CatStorageError> {
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
