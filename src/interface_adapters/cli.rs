use clap::{Args, CommandFactory, Parser, Subcommand};
use log::{error, info, warn};
use std::path::PathBuf;

use crate::{
    infrastructure::logger,
    use_cases::{
        helpers::{csv::ImportEntityCsvUseCase, model::ImportModelUseCase},
        ImportDeliverySlipUseCase, ImportInvoiceUseCase, ImportMappingClientUseCase,
        ImportOrderLineUseCase, ImportOrderUseCase, UseCaseError,
    },
};

// https://docs.rs/clap/latest/clap/_derive/_tutorial
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[command(subcommand)]
    action_command: ActionCommands,
}

#[derive(Debug, Subcommand)]
enum ActionCommands {
    /// Import Idempiere from CSV files
    Import(EntityCommand),
}

#[derive(Debug, Args)]
struct EntityCommand {
    /// Entity name
    #[command(subcommand)]
    entity: EntitySubCommand,
}

#[derive(Debug, Subcommand)]
pub enum EntitySubCommand {
    /// Import MappingClient Entities from Legacy Staging Database
    MappingClient(MandatoryArgs),

    /// Import Orders from CSV file defined in env file argument
    Order(MandatoryArgs),

    /// Import OrderLines from CSV file defined in env file argument
    Orderline(MandatoryArgs),

    // Import Delivery Slips from CSV file defined in env file argument
    DeliverySlip(MandatoryArgs),

    /// Import Invoices from CSV file defined in env file argument
    Invoice(MandatoryArgs),
}

#[derive(Debug, Args)]
pub struct MandatoryArgs {
    /// Sets env file
    #[arg(short, long, value_name = "ENV_FILE")]
    env_file: Option<PathBuf>,

    /// Batch mode
    #[arg(short = 'b', long)]
    batch: bool,

    /// Batch chunks size
    #[arg(short = 's', long, default_value = "100")]
    batch_size: usize,
}

pub fn main_using_clap() {
    let cli = Cli::parse();
    match cli.action_command {
        ActionCommands::Import(entity_command) => match entity_command.entity {
            EntitySubCommand::Order(arg) => {
                init(arg.env_file);
                if arg.batch {
                    info!("Batch mode not implemented yet");
                }
                info!("Importing orders...");
                error_logger(ImportOrderUseCase.execute());
                info!("Done");
            }
            EntitySubCommand::MappingClient(arg) => {
                init(arg.env_file);
                if arg.batch {
                    info!("Batch mode not implemented yet");
                }
                info!("Importing mapping clients...");
                error_logger(ImportMappingClientUseCase.execute());
                info!("Done");
            }
            EntitySubCommand::Orderline(arg) => {
                init(arg.env_file);
                info!("Importing order lines...");
                let result = ImportOrderLineUseCase::new().map(|mut handler| {
                    if arg.batch {
                        info!("Batch mode enabled - batch size: {}", arg.batch_size);
                        handler.set_batch(arg.batch_size);
                    }
                    error_logger(handler.execute());
                });
                if let Err(e) = result {
                    error_logger(Some(e));
                }

                info!("Done");
            }
            EntitySubCommand::DeliverySlip(arg) => {
                init(arg.env_file);
                info!("Importing delivery slips...");
                let mut handler = ImportDeliverySlipUseCase::default();
                if arg.batch {
                    info!("Batch mode enabled - batch size: {}", arg.batch_size);
                    handler.set_batch(arg.batch_size);
                }
                error_logger(handler.execute());
                info!("Done");
            }
            EntitySubCommand::Invoice(arg) => {
                init(arg.env_file);
                info!("Importing invoices...");
                let mut handler = ImportInvoiceUseCase::default();
                if arg.batch {
                    info!("Batch mode enabled - batch size: {}", arg.batch_size);
                    handler.set_batch(arg.batch_size);
                }
                error_logger(handler.execute());
                info!("Done");
            }
        },
    }
}

fn init(env_file: Option<PathBuf>) {
    info!("Load configuration...");
    // If --env--file argument is not provided, try to get .env file from the root of the crate
    if let Some(env_file_path) = env_file {
        if !env_file_path.exists() {
            exit(
                clap::error::ErrorKind::InvalidValue,
                format!("--env-file file does not exist {:?}", env_file_path).as_str(),
            );
        }
        dotenvy::from_path(env_file_path).expect("Unable to load env file");
    } else {
        println!("No --env-file argument provided, trying to load default .env file");
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

    // Init Logger when env file is loaded
    logger::init();
}

fn exit(kind: clap::error::ErrorKind, message: &str) {
    let mut cmd = Cli::command();
    cmd.error(kind, message).exit();
}

fn error_logger(errors: Option<Vec<UseCaseError>>) {
    if let Some(errors) = errors {
        for error in errors {
            match error {
                UseCaseError::Domain(e) => {
                    warn!("DomainError: {:?}", e);
                }
                UseCaseError::Infrastructure(e) => {
                    error!("InfrastructureError: {:?}", e);
                }
                UseCaseError::Mapping(e) => {
                    error!("MappingError: {:?}", e);
                }
            }
        }
    }
}
