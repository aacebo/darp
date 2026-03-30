mod location;
mod map;
mod span;

pub use location::*;
pub use map::*;
pub use span::*;

use std::{cell::RefCell, collections::BTreeMap};

use crate::template::Cursor;

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u32);

impl From<u32> for SourceId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for SourceId {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl std::fmt::Debug for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for SourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Primarily used to map spans (0 based character index ranges)
/// to bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// unique source id
    id: SourceId,

    /// raw source text
    text: String,

    /// line start offsets, in char units
    lines: Vec<usize>,

    /// Cache mapping character indices to UTF-8 byte offsets for efficient span slicing
    char_to_byte: RefCell<BTreeMap<usize, usize>>,
}

impl Source {
    pub fn new(id: SourceId, src: impl Into<String>) -> Self {
        let text = src.into();
        let mut lines = vec![0];
        let mut total = 0usize;

        for ch in text.chars() {
            total += 1;

            if ch == '\n' {
                lines.push(total);
            }
        }

        let mut char_to_byte = BTreeMap::new();
        char_to_byte.insert(0, 0);

        Self {
            id,
            text,
            lines,
            char_to_byte: RefCell::new(char_to_byte),
        }
    }

    pub fn id(&self) -> SourceId {
        self.id
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn begin(&self) -> Cursor {
        Cursor::from_src(self)
    }

    /// Resolves the given span into a byte index range.
    pub fn byte_range(&self, span: Span) -> std::ops::Range<usize> {
        let r = span.range();
        self.byte(r.start)..self.byte(r.end)
    }

    /// Gets a sub span of source text from the file.
    pub fn slice(&self, span: Span) -> &str {
        &self.text[self.byte_range(span)]
    }

    /// Resolves a global character index within this file into a 0-based `Location`.
    pub fn location(&self, i: usize) -> Location {
        let index = self.byte(i);

        match self.lines.binary_search(&i) {
            Err(next) => Location::new(index, next - 1, i - self.lines[next - 1]),
            Ok(line) => Location::new(index, line, 0),
        }
    }

    /// Returns the UTF-8 byte index corresponding to a global character index.
    pub fn byte(&self, i: usize) -> usize {
        let mut cache = self.char_to_byte.borrow_mut();

        if let Some(byte_index) = cache.get(&i) {
            return *byte_index;
        }

        let (&ci, &bi) = cache.range(..=i).next_back().unwrap();
        let mut char_index = ci;
        let mut byte_index = bi;

        #[allow(clippy::explicit_counter_loop)]
        for ch in self.text[bi..].chars() {
            if char_index == i {
                cache.insert(i, byte_index);
                return byte_index;
            }

            char_index += 1;
            byte_index += ch.len_utf8();
        }

        cache.insert(i, byte_index);
        byte_index
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
    }
}
