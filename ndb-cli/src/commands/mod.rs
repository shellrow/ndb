use std::path::PathBuf;

use clap::ArgMatches;

pub mod update;

pub enum AppCommands {
    Update,
    Default,
}

impl AppCommands {
    pub fn from_str(s: &str) -> AppCommands {
        match s {
            "update" => AppCommands::Update,
            _ => AppCommands::Default,
        }
    }
}

pub struct AppConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub dry_run: bool,
}

impl AppConfig {
    pub fn from_cli_arg(matches: &ArgMatches) -> Self {
        let input_dir = matches
            .get_one::<PathBuf>("input-dir")
            .cloned()
            .unwrap_or_default();
        let output_dir = matches
            .get_one::<PathBuf>("output-dir")
            .cloned()
            .unwrap_or_default();
        let dry_run = matches.get_flag("dry-run");
        AppConfig {
            input_dir,
            output_dir,
            dry_run,
        }
    }
}
