use actions::show_current::show_current;
use clap::Parser;

use crate::{
    actions::{
        delete::delete_tracking, report::show_report, start::start_tracking, stop::stop_tracking,
    },
    cli::Commands,
};

mod actions;
mod cli;
mod error;
mod file;

fn main() {
    let cli = cli::Cli::parse();

    let file_path = match cli.file {
        Some(f) => f,
        None => resolve_file_path(),
    };

    let res = match cli.command {
        Commands::Start(project) => start_tracking(project.project, file_path),
        Commands::Stop(project) => stop_tracking(project.project, file_path),
        Commands::Delete(project) => delete_tracking(project.project, file_path),
        Commands::Report { project, duration } => show_report(project.project, duration, file_path),
        Commands::ShowCurrent(project) => show_current(project.project, file_path),
    };

    if let Err(err) = res {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn resolve_file_path() -> String {
    std::env::var("TMC_FILE").unwrap_or(
        dirs::home_dir()
            .unwrap()
            .join("tmc_file.json")
            .to_str()
            .unwrap()
            .into(),
    )
}
