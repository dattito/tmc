#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("project \"{0}\" does not exist")]
    ProjectDoesNotExist(String),
    #[error("project \"{0}\" was not started")]
    ProjectWasNotYetStarted(String),
    #[error("project \"{0}\" is still running (stop it first)")]
    ProjectWasNotYetStopped(String),
    #[error("failed to (de)serialize json")]
    JSONSerialiazation(#[from] serde_json::Error),

    #[error("failed to read/write file")]
    IO(#[from] std::io::Error),
}

#[allow(dead_code)]
pub type Result = std::result::Result<(), Error>;
