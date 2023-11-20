use std::{fs::File, io::BufReader, path::Path};

use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Duration, DurationRound, Utc,
};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TmcFile {
    pub projects: Vec<TmcProject>,
}

impl TmcFile {
    pub fn from_file_or_new(path: &String) -> Result<Self, Error> {
        if Path::new(&path).exists() {
            let file = File::options().read(true).open(path)?;
            Ok(serde_json::from_reader(BufReader::new(file))?)
        } else {
            Ok(Self { projects: vec![] })
        }
    }

    pub fn write_to_file(&self, path: String) -> Result<(), Error> {
        let file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn create_project_if_not_exist(&mut self, name: &String) {
        if !self.projects.iter().any(|e| e.name == *name) {
            self.projects.push(TmcProject {
                name: name.clone(),
                timespans: vec![],
            })
        }
    }

    pub fn project(&mut self, name: &String) -> &TmcProject {
        self.create_project_if_not_exist(name);

        self.projects.iter().find(|e| e.name == *name).unwrap()
    }

    pub fn project_mut(&mut self, name: &String) -> &mut TmcProject {
        self.create_project_if_not_exist(name);

        self.projects.iter_mut().find(|e| e.name == *name).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmcProject {
    pub name: String,
    pub timespans: Vec<TmcTimespan>,
}

impl TmcProject {
    pub fn calculate_timespans_sum(&self, since: DateTime<Utc>) -> Duration {
        let mut d = Duration::seconds(0);

        self.timespans.iter().for_each(|ts| {
            let start = if ts.start < since { since } else { ts.start };

            let stop = match ts.stop {
                Some(s) => s,
                None => Utc::now().duration_round(Duration::seconds(1)).unwrap(),
            };

            if stop < since {
                return;
            }

            d = d + (stop - start);
        });

        d
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmcTimespan {
    #[serde(with = "ts_seconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub stop: Option<DateTime<Utc>>,
}
