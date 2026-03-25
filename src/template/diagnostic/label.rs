use crate::template::{Span, diagnostic::Level};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    span: Span,
    level: Level,
    message: Option<String>,
    suggestions: Vec<String>,
}

impl Label {
    pub fn new(span: Span) -> build::LabelBuilder {
        build::LabelBuilder {
            span,
            level: Level::Note,
            message: None,
            suggestions: vec![],
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn suggestions(&self) -> &[String] {
        &self.suggestions
    }
}

impl Span {
    pub fn label(&self) -> build::LabelBuilder {
        Label::new(*self)
    }
}

pub mod build {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct LabelBuilder {
        pub(super) span: Span,
        pub(super) level: Level,
        pub(super) message: Option<String>,
        pub(super) suggestions: Vec<String>,
    }

    impl LabelBuilder {
        pub fn level(mut self, level: Level) -> Self {
            self.level = level;
            self
        }

        pub fn message(mut self, message: impl std::fmt::Display) -> Self {
            self.message = Some(message.to_string());
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
