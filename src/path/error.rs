#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        Self { message: value }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl std::error::Error for ParseError {}
