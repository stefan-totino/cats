///
/// This module is intented to act as a layer of abstraction between modules
/// which interact with disk storage or memory, and the higher level
/// modules which interact with users or higher level programs
/// 
/// This layer of abstraction at the storage level is extremely powerful in terms of code testability,
/// modular development, scalability, redundancy, etc.
///
use crate::repository::cat_sql_store::{self, SqlStorage, SqlStorageError};
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
    Rusqlite(RusqliteError),
    SqlStorage(SqlStorageError),
}

/// The function signatures in this trait
/// provide a common interface for a client to interact with different types of storage mediums.
pub trait CatStorage {
    fn new(path: &'static str) -> Self;
    fn create(&self, name: String) -> Result<Cat, CatStorageError>;
    fn read(&self, id_opt: Option<String>) -> Result<Vec<Cat>, CatStorageError>;
    fn update(&self, id: String, new_cat: Cat) -> Result<Cat, CatStorageError>;
    fn delete(&self, id: String) -> Result<(), CatStorageError>;
}

impl CatStorage for SqlStorage {
    fn new(db_path: &'static str) -> SqlStorage {
        SqlStorage { db_path }
    }

    /// Creates a table called "cats" in the cats.db sqlite database file if necessary, 
    /// then proceeds to insert a new row of data into the table.
    /// Next it reads the newley inserted resource and returns a copy of it back up to the caller.
    fn create(&self, name: String) -> Result<Cat, CatStorageError> {
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::Rusqlite)?;
        let id = cat_sql_store::insert_cat(&name, &mut conn).map_err(CatStorageError::SqlStorage)?;
        match self.read(Some(id))?.first() {
            Some(cat) => Ok(Cat {
                id: cat.id.clone(),
                name: cat.name.clone(),
            }),
            None => Err(CatStorageError::MissingCat),
        }
    }

    /// If the cat's ID was supplied, expect one result in the output vector, else 
    /// expect as many as there are stored in the local cats.db.
    fn read(&self, id_opt: Option<String>) -> Result<Vec<Cat>, CatStorageError> {
        let cats = match id_opt {
            Some(id) => {
                let mut conn = Connection::open(self.db_path).map_err(CatStorageError::Rusqlite)?;
                cat_sql_store::select_cats_by_id(&id, &mut conn).map_err(CatStorageError::SqlStorage)?
            }
            None => {
                let mut conn = Connection::open(self.db_path).map_err(CatStorageError::Rusqlite)?;
                cat_sql_store::select_cats(&mut conn).map_err(CatStorageError::SqlStorage)?
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
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::Rusqlite)?;
        let update_count =
            cat_sql_store::update_column_by_id("cats", &id, "name", &cat.name, &mut conn)
                .map_err(CatStorageError::SqlStorage)?;

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
        let mut conn = Connection::open(self.db_path).map_err(CatStorageError::Rusqlite)?;
        cat_sql_store::delete_by_id("cats", &id, &mut conn).map_err(CatStorageError::SqlStorage)?;
        Ok(())
    }
}

/// If this were a production app, caching could be implemented here.
pub struct MemoryStorage {}

impl CatStorage for MemoryStorage {
    fn new(_path: &'static str) -> MemoryStorage {
        unimplemented!()
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

/// This notion of having more than one data store available via the same interface (i.e. CatStorage) 
/// is very powerful. For example, i can be cloning my data into an instance of MongoDB via a seperate 
/// process in preparation to roll clients over to it, all while still serving them on the SQL instance 
/// without any downtime or breaking API changes.
pub struct NoSqlStorage {}

impl CatStorage for NoSqlStorage {
    fn new(_path: &'static str) -> NoSqlStorage {
        unimplemented!()
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

/// This is what unit tests will work with. The implementation would have data statically defined
/// in it's memory scope.
pub struct MockStorage {}

impl CatStorage for MockStorage {
    fn new(_path: &'static str) -> MockStorage {
        unimplemented!()
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
