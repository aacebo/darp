use crate::template::{
    Span,
    source::{Location, Source},
};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor<'a> {
    src: &'a Source,
    index: usize,
    length: usize,
}

impl<'a> Cursor<'a> {
    pub fn from_src(src: &'a Source) -> Self {
        Self {
            src,
            index: 0,
            length: src.text().chars().count(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn location(&self) -> Location {
        self.src().location(self.index)
    }

    pub fn is_eof(&self) -> bool {
        self.len() == 0
    }

    pub fn src(&self) -> &Source {
        self.src
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn span(&self) -> Span {
        Span::new(self.src().id(), self.index, self.index + self.length)
    }

    pub fn remaining(&self) -> &str {
        self.src().slice(self.span())
    }

    pub fn advance(&self, n: usize) -> Self {
        if n > self.length {
            return Self {
                src: self.src,
                index: self.index + self.length,
                length: 0,
            };
        }

        Self {
            src: self.src,
            index: self.index + n,
            length: self.length - n,
        }
    }
}

impl<'a> std::hash::Hash for Cursor<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.src.id().hash(state);
        self.index.hash(state);
        self.length.hash(state);
    }
}
