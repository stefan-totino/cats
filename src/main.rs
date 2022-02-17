use cats_crud_cli_rust::{
    repository::cat_sql_store::SqlStorage,
    repository::cat_store::{Cat, CatStorage},
};
use clap::{Parser, Subcommand};
use rusqlite::Result;
use std::num::ParseIntError;

/// A simple CLI for CRUD operations revolving around cats.
#[derive(Parser, Debug)]
struct CommandLineInterface {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Name your new cat. If successful, a Cat structure will be displayed in text on the terminal
    Create { name: String },

    /// Optionally provide a cat’s ID, which you may have learned from the output of a previous command.
    /// If no ID is given, all cats in the system will be queried and reported.
    Read { id_opt: Option<String> },

    /// Provide the to be updated cat’s ID, along with a value for all of the current public facing properties of a cat.
    Update { id: String, name: String },

    /// Remove a cat from the system by providing it's ID.
    Delete { id: String },
}

#[derive(Debug)]
enum CommandLineInterfaceError {
    ParseInt(ParseIntError),
}

fn main() -> Result<(), CommandLineInterfaceError> {
    //  parse input from the command line into our CLI data models
    let args = CommandLineInterface::parse();

    // the cat_store module has been coded using a trait, this allows
    // for swappable storage sources at runtime. The left hand side of this
    // instantiation indicates which specific type of CatStorage will be invoked.
    let cat_storage: SqlStorage = CatStorage::new("./database/sqlite/cats.db");

    match &args.command {
        Commands::Create { name } => {
            match cat_storage.create(name.to_string()) {

                // if this were a production app, i would spend a lot of time thinking about
                // how my responses will drive their use of the app. i often lean on HATEOAS principals.
                // for this app i've kept it simple and just returned data instead of directions or messages.
                Ok(new_cat) => println!("{:?}", new_cat),

                // error handling is mapped to different types all the way down the call stack
                // using the question mark syntax. therefore errors printed here will have a stack
                // trace which can help infer the problem.
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
