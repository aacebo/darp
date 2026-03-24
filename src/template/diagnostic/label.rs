use crate::template::{Span, diagnostic::Level};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    span: Span,
    level: Level,
    message: String,
    suggestions: Vec<String>,
}

impl Label {
    pub fn new(span: Span, message: impl std::fmt::Display) -> build::LabelBuilder {
        build::LabelBuilder {
            span,
            level: Level::Note,
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

impl Span {
    pub fn label(&self, message: impl std::fmt::Display) -> build::LabelBuilder {
        Label::new(*self, message)
    }
}

pub mod build {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LabelBuilder {
        pub(super) span: Span,
        pub(super) level: Level,
        pub(super) message: String,
        pub(super) suggestions: Vec<String>,
    }

    impl LabelBuilder {
        pub fn level(mut self, level: Level) -> Self {
            self.level = level;
            self
        }

        pub fn suggestion(mut self, suggestion: impl std::fmt::Display) -> Self {
            self.suggestions.push(suggestion.to_string());
            self
        }

        pub fn build(self) -> Label {
            Label {
                span: self.span,
                level: self.level,
                message: self.message,
                suggestions: self.suggestions,
            }
        }
    }
}
