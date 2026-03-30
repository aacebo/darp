use crate::template::{
    Span,
    source::{Location, Source},
};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    src: *const Source,
    index: usize,
    length: usize,
}

impl Cursor {
    pub fn from_src(src: &Source) -> Self {
        Self {
            src: std::ptr::from_ref(src),
            index: 0,
            length: src.text().len(),
        }
    }

    pub fn src(&self) -> &Source {
        unsafe { self.src.as_ref_unchecked() }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn location(&self) -> Location {
        self.src().location(self.index)
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn span(&self) -> Span {
        Span::new(self.src().id(), self.index, self.index + self.length + 1)
    }

    pub fn as_str(&self) -> &str {
        self.src().slice(self.span())
    }

    /// Advance by `n` bytes, counting characters for the index.
    pub fn next_n(&self, n: usize) -> Self {
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

    pub fn next(&self) -> Self {
        self.next_n(1)
    }
}
