use crate::template::{Span, diagnostic::Level};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    span: Span,
    level: Level,
    message: String,
    suggestions: Vec<String>,
}

impl Label {
    pub fn new(span: Span, message: impl std::fmt::Display) -> Self {
        Self {
            span,
            level: Level::Unknown,
            message: message.to_string(),
            suggestions: vec![],
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn suggestions(&self) -> &[String] {
        &self.suggestions
    }
}
