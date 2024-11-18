use crate::git::parse::StatusHeader;

use super::{git_error::GitError, utils::status};
use std::{fmt, path::Path};

#[derive(Debug)]
pub struct GitStatus {
    /// The commit hash of the last commit Header:oid
    commit: String,
    /// The local branch name Header:head
    branch: String,
    /// The remote branch name Header:upstream
    upstream: Option<String>,
    /// The number of commits ahead of the remote branch Header:ab (+)
    ahead: i32,
    /// The number of commits behind the remote branch Header:ab (-)
    behind: i32,
    /// Tracked files that have changed
    tracked_changes: Vec<String>,
    /// Untracked files
    untracked: Vec<String>,
}

impl GitStatus {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, GitError> {
        let path = path.as_ref();
        let status = status(path)?;
        Self::parse(&status).ok_or(GitError::ParseFailed)
    }

    pub fn commit(&self) -> &str {
        &self.commit
    }

    pub fn branch(&self) -> &str {
        &self.branch
    }

    pub fn upstream(&self) -> Option<&str> {
        self.upstream.as_deref()
    }

    pub fn ahead(&self) -> i32 {
        self.ahead
    }

    pub fn behind(&self) -> i32 {
        self.behind
    }

    pub fn tracked_changes(&self) -> &[String] {
        &self.tracked_changes
    }

    pub fn untracked(&self) -> &[String] {
        &self.untracked
    }

    pub fn num_tracked(&self) -> usize {
        self.tracked_changes.len()
    }

    pub fn num_untracked(&self) -> usize {
        self.untracked.len()
    }

    pub fn total_changes(&self) -> usize {
        self.num_tracked() + self.num_untracked()
    }

    pub fn status_line(&self) -> String {
        format!(
            "[{}: ↑{} ↓{} | {} change{}]",
            self.branch,
            self.ahead,
            self.behind,
            self.total_changes(),
            if self.total_changes() > 1 { "s" } else { "" }
        )
    }

    fn parse(status: &str) -> Option<Self> {
        let mut oid = None;
        let mut branch = None;
        let mut upstream = None;
        let mut ahead = None;
        let mut behind = None;
        let mut tracked_changes: Vec<String> = Vec::new();
        let mut untracked: Vec<String> = Vec::new();

        for line in status.lines() {
            if line.starts_with('#') {
                match StatusHeader::parse(line)? {
                    StatusHeader::Oid(commit) => oid = Some(commit),
                    StatusHeader::Head(b) => branch = Some(b),
                    StatusHeader::Upstream(up) => upstream = Some(up),
                    StatusHeader::Ab(a, b) => {
                        ahead = Some(a);
                        behind = Some(b);
                    }
                }
            } else if line.starts_with('1') || line.starts_with('2') {
                tracked_changes.push(line.split(' ').last()?.to_string());
            } else if line.starts_with('?') {
                untracked.push(line.split(' ').last()?.to_string());
            }
        }
        Some(Self {
            commit: oid?,
            branch: branch?,
            upstream,
            ahead: ahead.unwrap_or(0),
            behind: behind.unwrap_or(0),
            tracked_changes,
            untracked,
        })
    }
}

impl fmt::Display for GitStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Commit: {}", self.commit)?;
        writeln!(f, "Branch: {}", self.branch)?;
        if let Some(upstream) = &self.upstream {
            writeln!(f, "Upstream: {}", upstream)?;
            writeln!(f, "Ahead: {}", self.ahead)?;
            writeln!(f, "Behind: {}", self.behind)?;
        }
        writeln!(f, "Tracked changes:")?;
        for change in &self.tracked_changes {
            writeln!(f, "  {}", change)?;
        }
        writeln!(f, "Untracked files:")?;
        for file in &self.untracked {
            writeln!(f, "  {}", file)?;
        }
        Ok(())
    }
}
