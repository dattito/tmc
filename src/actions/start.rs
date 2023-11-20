use chrono::{Duration, DurationRound, Utc};

use crate::{
    error::{Error, Result},
    file::{TmcFile, TmcTimespan},
};

pub fn start_tracking(project_name: String, file_path: String) -> Result {
    let mut tmc = TmcFile::from_file_or_new(&file_path)?;

    let project = tmc.project_mut(&project_name);

    if let Some(ts) = project.timespans.last() {
        if ts.stop.is_none() {
            return Err(Error::ProjectWasNotYetStopped(project_name));
        }
    }

    project.timespans.push(TmcTimespan {
        start: Utc::now().duration_round(Duration::seconds(1)).unwrap(),
        stop: None,
    });

    tmc.write_to_file(file_path)?;

    Ok(())
}
