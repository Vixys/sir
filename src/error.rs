#[derive(Debug)]
pub struct RCatError {
    source: String,
    reason: String,
}

impl RCatError {
    pub fn new(source: String, reason: String) -> RCatError {
        RCatError { source, reason }
    }
}

impl std::fmt::Display for RCatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rcat: {}: {}", self.source, self.reason)
    }
}
