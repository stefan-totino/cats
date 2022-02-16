use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

/*
    router
*/

fn main() -> Result<()> {
    let args = CommandLineInterface::parse();
    match &args.command {
        Commands::Create { name } => {
            let cat = Cat {
                name: name.to_string(),
            };
            let path = "database/sqlite/cats.db";
            let mut conn = Connection::open(path)?;
            let id = insert_into_cats_tx(&cat, &mut conn)?;
            println!("created a cat with id: {:?}", id);
            Ok(())
        }
        Commands::Read { id } => {
            let cats = match id {
                Some(value) => {
                    let path = "database/sqlite/cats.db";
                    let mut conn = Connection::open(path)?;
                    select_cats_by_id(value.to_string(), &mut conn)?
                }
                None => {
                    let path = "database/sqlite/cats.db";
                    let mut conn = Connection::open(path)?;
                    select_cats(&mut conn)?
                }
            };
            println!("read the following cats: {:?}", cats);
            Ok(())
        }
        Commands::Update { id, name } => {
            let path = "database/sqlite/cats.db";
            let mut conn = Connection::open(path)?;
            let update_count = update_cats_tx(id.to_string(), name.to_string(), &mut conn)?;
            println!("updated {:?} cats", update_count);
            Ok(())
        }
        Commands::Delete { id } => {
            let path = "database/sqlite/cats.db";
            let mut conn = Connection::open(path)?;
            delete_cats_by_id_tx(id.to_string(), &mut conn)?;
            println!("deleted cat with id: {:?}", id);
            Ok(())
        }
    }
}

/*
    data models
*/

#[derive(Parser, Debug)]
struct CommandLineInterface {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create { name: String },
    Read { id: Option<String> },
    Update { id: String, name: String },
    Delete { id: String },
}

#[derive(Debug)]
struct Cat {
    name: String,
}

/*
    sqlite commands
*/

fn insert_into_cats_tx(cat: &Cat, conn: &mut Connection) -> Result<String> {
    let tx = conn.transaction()?;
    tx.execute(
        "CREATE TABLE IF NOT EXISTS cats (
            id integer PRIMARY KEY,
            name text NOT NULL
        )",
        [],
    )?;
    tx.execute("INSERT INTO cats (name) VALUES (?1)", [&cat.name])?;
    let last_id = tx.last_insert_rowid().to_string();

    tx.commit()?;

    Ok(last_id)
}

fn select_cats_by_id(id: String, conn: &mut Connection) -> Result<Vec<Cat>> {
    let mut statement = conn.prepare("SELECT * FROM cats WHERE id = ?")?;
    let mut rows = statement.query(rusqlite::params![id])?;
    let mut cats = Vec::new();
    while let Some(row) = rows.next()? {
        cats.push(Cat { name: row.get(1)? });
    }

    Ok(cats)
}

fn select_cats(conn: &mut Connection) -> Result<Vec<Cat>> {
    let mut statement = conn.prepare("SELECT * FROM cats")?;
    let mut rows = statement.query([])?;
    let mut cats = Vec::new();
    while let Some(row) = rows.next()? {
        cats.push(Cat { name: row.get(1)? });
    }

    Ok(cats)
}

fn update_cats_tx(id: String, name: String, conn: &mut Connection) -> Result<usize> {
    let tx = conn.transaction()?;
    let rows_updated = tx.execute("UPDATE cats SET name = (?1) WHERE id = (?2)", [name, id])?;
    tx.commit()?;

    Ok(rows_updated)
}

fn delete_cats_by_id_tx(id: String, conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM cats WHERE id = (?1)", [id])?;
    tx.commit()?;

    Ok(())
}
