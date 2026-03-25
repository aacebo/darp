use crate::template::{Span, source::{Source, SourceId}};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cursor {
    src_id: SourceId,
    text: *const str,
    index: usize,
}

impl Cursor {
    pub fn new(src: &Source) -> Self {
        Self {
            src_id: src.id(),
            text: src.text(),
            index: 0,
        }
    }

    pub fn text(&self) -> &str {
        unsafe { &*self.text }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn first(&self) -> Option<char> {
        self.text().chars().next()
    }

    pub fn span(&self) -> Span {
        Span::new(self.src_id, self.index, self.index + 1)
    }

    pub fn span_to(&self, end: &Self) -> Span {
        assert!(self.src_id == end.src_id);
        Span::new(self.src_id, self.index, end.index)
    }

    /// Advance by `n` bytes, counting characters for the index.
    pub fn advance(&self, n: usize) -> Self {
        let consumed = &self.text()[..n];
        let chars = consumed.chars().count();

        Self {
            src_id: self.src_id,
            text: &self.text()[n..],
            index: self.index + chars,
        }
    }

    /// Advance while predicate holds on chars.
    pub fn skip_while(&self, mut pred: impl FnMut(char) -> bool) -> Self {
        let mut bytes = 0;

        for ch in self.text().chars() {
            if !pred(ch) {
                break;
            }

            bytes += ch.len_utf8();
        }

        self.advance(bytes)
    }

    pub fn skip_whitespace(mut self) -> Self {
        loop {
            // Whitespace
            let next = self.skip_while(|ch| ch.is_whitespace());

            if next.index() != self.index() {
                self = next;
                continue;
            }

            // Line comment
            if self.starts_with("//") {
                self = self.skip_while(|ch| ch != '\n');

                if self.starts_with("\n") {
                    self = self.advance(1);
                }

                continue;
            }

            // Block comment (nested)
            if self.starts_with("/*") {
                match self.skip_comment() {
                    None => break, // unterminated — let the main parser deal with it
                    Some(next) => {
                        self = next;
                        continue;
                    }
                }
            }

            break;
        }

        self
    }

    pub fn skip_comment(&self) -> Option<Self> {
        let mut cur = self.advance(2); // skip /*
        let mut depth = 1u32;

        while !cur.is_empty() {
            if cur.starts_with("/*") {
                depth += 1;
                cur = cur.advance(2);
            } else if cur.starts_with("*/") {
                depth -= 1;
                cur = cur.advance(2);

                if depth == 0 {
                    return Some(cur);
                }
            } else {
                let ch = cur.first().unwrap();
                cur = cur.advance(ch.len_utf8());
            }
        }

        None
    }
}

impl std::ops::Deref for Cursor {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::hash::Hash for Cursor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}
