mod code;
mod label;
mod level;

pub use code::*;
pub use label::*;
pub use level::*;

use crate::template::Span;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    span: Span,
    code: Code,
    message: Option<String>,
    labels: Vec<Label>,
}

impl Diagnostic {
    pub fn new(span: Span, code: Code) -> build::DiagnosticBuilder {
        build::DiagnosticBuilder {
            span,
            code,
            message: None,
            labels: vec![],
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn code(&self) -> Code {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_deref().unwrap_or(self.code.message)
    }

    pub fn labels(&self) -> &[Label] {
        &self.labels
    }
}

impl Span {
    pub fn emit(&self, code: Code) -> build::DiagnosticBuilder {
        Diagnostic::new(*self, code)
    }
}

pub mod build {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct DiagnosticBuilder {
        pub(super) span: Span,
        pub(super) code: Code,
        pub(super) message: Option<String>,
        pub(super) labels: Vec<Label>,
    }

    impl DiagnosticBuilder {
        pub fn message(mut self, message: impl std::fmt::Display) -> Self {
            self.message = Some(message.to_string());
            self
        }

        pub fn label(mut self, label: Label) -> Self {
            self.labels.push(label);
            self
        }

        pub fn labels(mut self, labels: Vec<Label>) -> Self {
            self.labels = labels;
            self
        }

        pub fn build(self) -> Diagnostic {
            Diagnostic {
                span: self.span,
                code: self.code,
                message: self.message,
                labels: self.labels,
            }
        }
    }
}
