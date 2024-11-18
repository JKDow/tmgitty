use super::{git_error::GitError, utils::status};
use std::path::Path;

#[derive(Debug)]
enum StatusHeader {
    /// Commit hash
    Oid(String),
    /// Local branch
    Head(String),
    /// Upstream branch
    Upstream(String),
    /// (Ahead, Behind)
    Ab(i32, i32),
}

pub struct GitStatus {
    /// The commit hash of the last commit Header:oid
    pub commit: String,
    /// The local branch name Header:head
    pub branch: String,
    /// The remote branch name Header:upstream
    pub upstream: Option<String>,
    /// The number of commits ahead of the remote branch Header:ab (+)
    pub ahead: i32,
    /// The number of commits behind the remote branch Header:ab (-)
    pub behind: i32,
    pub staged: Vec<String>,
    pub not_staged: Vec<String>,
    pub untracked: Vec<String>,
}

impl GitStatus {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, GitError> {
        let path = path.as_ref();
        let status = status(path)?;
        Ok(Self::parse(&status).unwrap())
    }

    fn parse(status: &str) -> Option<Self> {
        let mut headers: Vec<StatusHeader> = Vec::new();

        for line in status.lines() {
            if line.starts_with('#') {
                if let Some(h) = Self::parse_header(line) {
                    println!("{:?}", h);
                    headers.push(h);
                }
            }
        }
        todo!()
    }

    fn parse_header(line: &str) -> Option<(StatusHeader)> {
        let parts: Vec<&str> = line.split(' ').collect();
        let key = parts[1];
        match key {
            "branch.oid" => Some(StatusHeader::Oid(parts.get(2)?.to_string())),
            "branch.head" => Some(StatusHeader::Head(parts.get(2)?.to_string())),
            "branch.upstream" => Some(StatusHeader::Upstream(parts.get(2)?.to_string())),
            "branch.ab" => {
                // in format "+a -b" where a and b are integers
                let a = parts.get(2)?.parse().ok()?;
                let b = parts.get(3)?.parse().ok()?;
                Some(StatusHeader::Ab(a, b))
            }
            _ => None,
        }
    }
}
