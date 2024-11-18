#[derive(Debug)]
pub(crate) enum StatusHeader {
    /// Commit hash
    Oid(String),
    /// Local branch
    Head(String),
    /// Upstream branch
    Upstream(String),
    /// (Ahead, Behind)
    Ab(i32, i32),
}

impl StatusHeader {
    pub fn parse(line: &str) -> Option<StatusHeader> {
        let parts: Vec<&str> = line.split(' ').collect();
        match *parts.get(1)? {
            "branch.oid" => Some(StatusHeader::Oid(parts.get(2)?.to_string())),
            "branch.head" => Some(StatusHeader::Head(parts.get(2)?.to_string())),
            "branch.upstream" => Some(StatusHeader::Upstream(parts.get(2)?.to_string())),
            "branch.ab" => {
                let a = parts.get(2)?.parse().ok()?;
                let b = parts.get(3)?.parse().ok()?;
                Some(StatusHeader::Ab(a, b))
            }
            _ => None,
        }
    }
}
