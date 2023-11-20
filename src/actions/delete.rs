use crate::{error::Result, file::TmcFile};

pub fn delete_tracking(project_name: String, file_path: String) -> Result {
    let mut tmc = TmcFile::from_file_or_new(&file_path)?;

    if let Some(i) = tmc.projects.iter().position(|e| e.name == project_name) {
        tmc.projects.remove(i);
    }

    tmc.write_to_file(file_path)?;

    Ok(())
}
