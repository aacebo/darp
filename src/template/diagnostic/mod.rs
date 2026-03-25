mod code;
mod label;
mod level;

pub use code::*;
pub use label::*;
pub use level::*;

use crate::template::Span;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn level(&self) -> Level {
        let mut level = Level::Note;

        for diagnostic in &self.0 {
            if diagnostic.code.level > level {
                level = diagnostic.code.level;
            }
        }

        level
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.0.iter()
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) -> &mut Self {
        self.0.push(diagnostic);
        self
    }
}

impl AsRef<[Diagnostic]> for Diagnostics {
    fn as_ref(&self) -> &[Diagnostic] {
        &self.0
    }
}

impl IntoIterator for Diagnostics {
    type IntoIter = std::vec::IntoIter<Diagnostic>;
    type Item = Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Diagnostic> for Diagnostics {
    fn extend<T: IntoIterator<Item = Diagnostic>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

        pub fn labels(mut self, labels: impl IntoIterator<Item = Label>) -> Self {
            self.labels.extend(labels);
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
