use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Failed to run git command: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to fetch from remote repository")]
    FetchFailed,
    #[error("Failed to get status from repository")]
    StatusFailed,
    #[error("Path is not a git repository: {0}")]
    NoRepository(String),
    #[error("Failed to parse git status output")]
    ParseFailed,
}
