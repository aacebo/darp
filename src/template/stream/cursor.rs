use crate::template::{
    Span,
    source::{Source, SourceId},
};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    src_id: SourceId,
    index: usize,
    length: usize,
}

impl Cursor {
    pub fn new(src: &Source) -> Self {
        Self {
            src_id: src.id(),
            index: 0,
            length: src.text().len(),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn span(&self) -> Span {
        Span::new(self.src_id, self.index, self.index + 1)
    }

    /// Advance by `n` bytes, counting characters for the index.
    pub fn next_n(&self, n: usize) -> Self {
        if n > self.length {
            return Self {
                src_id: self.src_id,
                index: self.index + self.length,
                length: 0,
            };
        }

        Self {
            src_id: self.src_id,
            index: self.index + n,
            length: self.length - n,
        }
    }

    pub fn next(&self) -> Self {
        self.next_n(1)
    }
}
