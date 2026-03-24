use crate::template::diagnostic::Level;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Code {
    pub id: &'static str,
    pub level: Level,
    pub message: &'static str,
}

impl Code {
    pub const UNEXPECTED_TOKEN: Self = Self::error("C001", "unexpected token");
    pub const UNTERMINATED_BLOCK: Self = Self::error("C002", "unterminated block");
    pub const INVALID_EXPR: Self = Self::error("C003", "invalid expression");
    pub const MISSING_CLOSING_DELIM: Self = Self::error("C004", "missing closing delimiter");
}

impl Code {
    pub const fn new(id: &'static str, level: Level, message: &'static str) -> Self {
        Self { id, level, message }
    }

    pub const fn note(id: &'static str, message: &'static str) -> Self {
        Self::new(id, Level::Note, message)
    }

    pub const fn help(id: &'static str, message: &'static str) -> Self {
        Self::new(id, Level::Help, message)
    }

    pub const fn warn(id: &'static str, message: &'static str) -> Self {
        Self::new(id, Level::Warning, message)
    }

    pub const fn error(id: &'static str, message: &'static str) -> Self {
        Self::new(id, Level::Error, message)
    }
}
