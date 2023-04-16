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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_succeed() {
        let result = RCatError::new("some_source".to_string(), "some_error".to_string());
        assert_eq!(result.source, "some_source".to_string());
        assert_eq!(result.reason, "some_error".to_string());
    }

    #[test]
    fn fmt_expected_output() {
        let error = RCatError::new("some_source".to_string(), "some_error".to_string());

        let result = format!("{}", error);

        assert_eq!(result, "rcat: some_source: some_error".to_string());
    }
}
