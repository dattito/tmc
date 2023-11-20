use chrono::{Duration, DurationRound, Utc};

use crate::error::{Error, Result};
use crate::file::TmcFile;
use colored::Colorize;

pub fn show_current(project_name: String, file_path: String) -> Result {
    let mut tmc = TmcFile::from_file_or_new(&file_path)?;

    let project = tmc.project_mut(&project_name);

    let Some(ts) = project.timespans.last() else {
        return Err(Error::ProjectWasNotYetStarted(project_name));
    };

    match ts.stop {
        Some(_) => println!("project {project_name} is currently not active"),
        None => {
            let d = humantime::format_duration(
                Utc::now()
                    .duration_round(Duration::seconds(1))
                    .unwrap()
                    .signed_duration_since(ts.start)
                    .to_std()
                    .unwrap(),
            )
            .to_string()
            .bold()
            .underline();

            println!("project \"{project_name}\" has been running since {d}")
        }
    }

    Ok(())
}
