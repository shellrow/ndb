mod commands;

use std::path::PathBuf;

use clap::{crate_description, crate_name, crate_version, value_parser};
use clap::{Arg, ArgMatches, Command};
use commands::AppCommands;
use anyhow::Result;
use tracing::Level;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Parse command line arguments
    let args: ArgMatches = parse_args();
    
    // Initialize logger
    let verbose = args.get_flag("verbose");
    init_logger(verbose);

    let subcommand_name = args.subcommand_name().unwrap_or("");
    let app_command = AppCommands::from_str(subcommand_name);

    let sub_matches = args.subcommand_matches(subcommand_name).unwrap_or(&args);
    let config = commands::AppConfig::from_cli_arg(&sub_matches);

    match app_command {
        AppCommands::Update => commands::update::update_bin_db(config),
        AppCommands::Default => {
            println!("Unknown command: {}", subcommand_name);
            println!("Use --help to see available commands.");
            Ok(())
        }
    }
}

pub fn init_logger(verbose: bool) {
    let level = if verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    // Init logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_timer(ChronoLocal::rfc_3339())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn parse_args() -> ArgMatches {
    let app: Command = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("verbose")
                .help("Run in verbose mode")
                .long("verbose")
                .num_args(0)
                .required(false)
        )
        // Sub-command for update database.
        .subcommand(Command::new("update")
            .about("Update the bin database with the latest csv data")
            .arg(
                Arg::new("dry-run")
                    .help("Run in dry-run mode, no changes will be made")
                    .long("dry-run")
                    .num_args(0)
                    .required(false)
            )
            .arg(Arg::new("input-dir")
                .help("Directory containing the latest CSV files")
                .short('i')
                .long("input-dir")
                .value_name("dir_path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(Arg::new("output-dir")
                .help("Directory to output the updated BIN DB files")
                .short('o')
                .long("output-dir")
                .value_name("dir_path")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
        )
        ;
    app.get_matches()
}
