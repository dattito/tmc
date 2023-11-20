use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_parser=clap::builder::NonEmptyStringValueParser::new())]
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start tracking
    Start(#[command(flatten)] Project),
    /// Stop tracking
    Stop(#[command(flatten)] Project),
    /// Show report
    Report {
        /// Which timeframe to show
        #[arg(short, long)]
        duration: ReportDuration,

        #[command(flatten)]
        project: Project,
    },

    /// Delete a project
    Delete(#[command(flatten)] Project),

    /// Show when tracking started
    ShowCurrent(#[command(flatten)] Project),
}

#[derive(Debug, Args)]
pub struct Project {
    /// The name of the project
    #[arg(short, long, value_parser=clap::builder::NonEmptyStringValueParser::new())]
    pub project: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ReportDuration {
    Day,
    TwentyFourHours,
    ThisWeek,
    SevenDays,
    CalenderMonth,
    ThirtyDays,
    AllTime,
}
