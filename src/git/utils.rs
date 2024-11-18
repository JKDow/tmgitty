use std::{path::Path, process::Command};
use super::git_error::GitError;

/// Runs git fetch and git status in the given path.
pub fn fetch_and_status(path: impl AsRef<Path>) -> Result<String, GitError> {
    let path = path.as_ref();
    fetch(path)?;
    status(path)
}

/// Fetches the latest changes from the remote repository.
/// Returns `Ok(())` if the fetch was successful, otherwise returns an error.
pub fn fetch(path: impl AsRef<Path>) -> Result<(), GitError> {
    let path = path.as_ref();
    if !is_git_repo(path) {
        return Err(GitError::NoRepository(path.to_string_lossy().to_string()));
    }
    let status = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("fetch")
        .arg("--quiet")
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(GitError::FetchFailed)
    }
}

/// Runs git status in the given path.
/// Returns the output of the command in a string
pub fn status(path: impl AsRef<Path>) -> Result<String, GitError> {
    let path = path.as_ref();
    if !is_git_repo(path) {
        return Err(GitError::NoRepository(path.to_string_lossy().to_string()));
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("--branch")
        .arg("--porcelain=2")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(GitError::StatusFailed)
    }
}

/// Looks for a `.git` directory in the given path.
fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists()
}
