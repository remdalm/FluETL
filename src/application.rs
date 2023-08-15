mod use_cases;

use clap::{Args, CommandFactory, Parser, Subcommand};
use std::path::PathBuf;
use use_cases::UseCase;

// https://docs.rs/clap/latest/clap/_derive/_tutorial
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Sets env file
    #[arg(short, long, value_name = "ENV_FILE")]
    env_file: Option<PathBuf>,

    #[command(subcommand)]
    action_command: Option<ActionCommands>,
}

#[derive(Debug, Subcommand)]
enum ActionCommands {
    /// Import Idempiere from CSV files
    Import(EntityCommand),
}

#[derive(Debug, Args)]
struct EntityCommand {
    /// Entity name
    #[clap(subcommand)]
    entity: EntitySubCommand,
}

#[derive(Debug, Subcommand)]
pub enum EntitySubCommand {
    /// Import MappingClient from CSV file defined in env file argument
    MappingClient,

    /// Import Order from CSV file defined in env file argument
    Order,

    /// Import OrderLine from CSV file defined in env file argument
    Orderline,
}

pub fn main_using_clap() {
    let cli = Cli::parse();

    // If --env--file argument is not provided, try to get .env file from the root of the crate
    if let Some(env_file_path) = cli.env_file {
        if !env_file_path.exists() {
            exit(
                clap::error::ErrorKind::InvalidValue,
                format!("--env-file file does not exist {:?}", env_file_path).as_str(),
            );
        }
        dotenvy::from_path(env_file_path).expect("Unable to load env file");
    } else {
        let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let default_env_file = root_path.join(".env");
        if default_env_file.exists() {
            dotenvy::from_path(default_env_file).expect("Unable to load default env file");
        } else {
            exit(
                clap::error::ErrorKind::MissingRequiredArgument,
                "--env-file argument or a default .env at the root of the binary file is mandatory",
            );
        }
    }

    if let Some(action_command) = cli.action_command {
        match action_command {
            ActionCommands::Import(entity_command) => match entity_command.entity {
                EntitySubCommand::Order => {
                    println!("Importing orders...");
                    use_cases::import_orders::ImportOrderUseCase.execute();
                }
                other => {
                    exit(
                        clap::error::ErrorKind::InvalidValue,
                        format!("{:?} is not yet implemented", other).as_str(),
                    );
                }
            },
        }
    }
}

fn exit(kind: clap::error::ErrorKind, message: &str) {
    let mut cmd = Cli::command();
    cmd.error(kind, message).exit();
}
