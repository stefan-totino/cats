use cats_crud_cli_rust::{
    repository::cat_store::{Cat, CatStorage},
    repository::sql_store::SqlStorage,
};
use clap::{Parser, Subcommand};
use rusqlite::Result;
use std::num::ParseIntError;

#[derive(Parser, Debug)]
struct CommandLineInterface {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create { name: String },
    Read { id_opt: Option<String> },
    Update { id: String, name: String },
    Delete { id: String },
}

#[derive(Debug)]
enum CommandLineInterfaceError {
    ParseInt(ParseIntError),
}

fn main() -> Result<(), CommandLineInterfaceError> {
    let args = CommandLineInterface::parse();
    let cat_storage: SqlStorage = CatStorage::new("./database/sqlite/cats.db");
    match &args.command {
        Commands::Create { name } => {
            match cat_storage.create(name.to_string()) {
                Ok(new_cat) => println!("{:?}", new_cat),
                Err(err) => println!("{:?}", err),
            };

            Ok(())
        }
        Commands::Read { id_opt } => {
            match cat_storage.read(id_opt.to_owned()) {
                Ok(cats) => println!("{:?}", cats),
                Err(err) => println!("{:?}", err),
            }

            Ok(())
        }
        Commands::Update { id, name } => {
            let updated_cat = Cat {
                id: id
                    .parse::<i32>()
                    .map_err(CommandLineInterfaceError::ParseInt)?,
                name: name.to_string(),
            };
            match cat_storage.update(id.to_owned(), updated_cat) {
                Ok(updated_cat) => println!("{:?}", updated_cat),
                Err(err) => println!("{:?}", err),
            }

            Ok(())
        }
        Commands::Delete { id } => {
            match cat_storage.delete(id.to_owned()) {
                Ok(()) => println!("The cat who had an id of [{:?}] has been deleted!", id),
                Err(err) => println!("{:?}", err),
            }

            Ok(())
        }
    }
}
