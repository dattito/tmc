use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
use colored::Colorize;

use crate::{cli::ReportDuration, error::Result, file::TmcFile};

pub fn show_report(project_name: String, duration: ReportDuration, file_path: String) -> Result {
    let mut tmc = TmcFile::from_file_or_new(&file_path)?;

    let project = tmc.project(&project_name);

    let since: DateTime<Local> = match duration {
        ReportDuration::Day => Local::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap(),
        ReportDuration::TwentyFourHours => Local::now() - Duration::days(1),
        ReportDuration::ThisWeek => (Local::now()
            - Duration::days(Local::now().weekday().number_from_monday().into()))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap(),
        ReportDuration::SevenDays => Local::now() - Duration::days(7),
        ReportDuration::CalenderMonth => Local
            .with_ymd_and_hms(Local::now().year(), Local::now().month(), 1, 0, 0, 0)
            .unwrap(),
        ReportDuration::ThirtyDays => Local::now() - Duration::days(30),
        ReportDuration::AllTime => DateTime::from_timestamp(0i64, 0u32).unwrap().into(),
    };

    println!(
        "Report for project {} with timeframe {:?}: {}",
        project_name,
        duration,
        humantime::format_duration(
            project
                .calculate_timespans_sum(since.into())
                .to_std()
                .unwrap()
        )
        .to_string()
        .bold()
        .underline()
    );

    Ok(())
}
