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
    pub fn new(span: Span, code: Code) -> Self {
        Self {
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

pub mod build {}
