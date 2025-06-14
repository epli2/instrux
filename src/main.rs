mod formats;
mod model;
mod opts;
mod diff;
mod commands {
    pub mod diff;
    pub mod generate;
    pub mod init;
    pub mod merge;
    pub mod validate;
}

use clap::Parser;
use opts::{Cli, Commands};
// Will be used in the command implementations
// use formats::{ToFormat, FromFormat};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            dry_run,
            force,
            watch,
        } => {
            commands::generate::run(dry_run, force, watch);
        }
        Commands::Init { from } => {
            commands::init::run(from);
        }
        Commands::Merge { from } => {
            commands::merge::run(from);
        }
        Commands::Diff { tool } => {
            commands::diff::run(tool);
        }
        Commands::Validate => {
            commands::validate::run();
        }
    }
}
