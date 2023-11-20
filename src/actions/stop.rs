use chrono::{Duration, DurationRound, Utc};

use crate::{
    error::{Error, Result},
    file::TmcFile,
};

pub fn stop_tracking(project_name: String, file_path: String) -> Result {
    let mut tmc = TmcFile::from_file_or_new(&file_path)?;

    let project = tmc.project_mut(&project_name.clone());

    if let Some(ts) = project.timespans.last() {
        if ts.stop.is_some() {
            return Err(Error::ProjectWasNotYetStarted(project_name));
        }
    } else {
        return Err(Error::ProjectWasNotYetStarted(project_name));
    }

    project.timespans.last_mut().unwrap().stop =
        Some(Utc::now().duration_round(Duration::seconds(1)).unwrap());

    tmc.write_to_file(file_path)?;

    Ok(())
}
