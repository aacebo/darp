use crate::template::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    span: Span,
    inner: Box<str>,
}

impl Ident {
    pub fn new(span: Span, inner: &str) -> Self {
        Self {
            span,
            inner: inner.to_string().into_boxed_str(),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
