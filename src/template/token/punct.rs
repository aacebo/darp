use crate::template::{Span, token::Spacing};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Punct {
    span: Span,
    spacing: Spacing,
    inner: char,
}

impl Punct {
    pub fn new(span: Span, spacing: Spacing, inner: char) -> Self {
        Self {
            inner,
            spacing,
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn inner(&self) -> char {
        self.inner
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
