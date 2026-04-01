use crate::template::{
    Span,
    source::{Location, Source},
};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor<'a> {
    source: &'a Source,
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self { source, index: 0 }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn location(&self) -> Location {
        self.source.location(self.index)
    }

    pub fn is_eof(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.source.text().chars().count() - self.index
    }

    pub fn span(&self) -> Span {
        Span::new(self.source.id(), self.index, self.index + self.len())
    }

    pub fn remaining(&self) -> &str {
        self.source.slice(self.span())
    }

    pub fn advance(&self, n: usize) -> Self {
        if n > self.len() {
            return Self {
                source: self.source,
                index: self.index + self.len(),
            };
        }

        Self {
            source: self.source,
            index: self.index + n,
        }
    }
}

impl<'a> std::hash::Hash for Cursor<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.id().hash(state);
        self.index.hash(state);
    }
}
