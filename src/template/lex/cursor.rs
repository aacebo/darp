use crate::template::{Diagnostics, Span, lex::ScanResult, source::SourceId};

/// Zero-copy immutable cursor over source text.
/// Each parse step returns a new advanced cursor.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    src_id: SourceId,
    rest: &'a str,
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src_id: SourceId, src: &'a str, offset: usize) -> Self {
        Self {
            src_id,
            rest: src,
            offset,
        }
    }

    pub fn src_id(&self) -> SourceId {
        self.src_id
    }

    pub fn rest(&self) -> &'a str {
        self.rest
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn first(&self) -> Option<char> {
        self.rest.chars().next()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.rest.starts_with(s)
    }

    pub fn span(&self) -> Span {
        Span::new(self.src_id, self.offset, self.offset + 1)
    }

    pub fn ok<T>(&self, value: T) -> ScanResult<'_, T> {
        ScanResult {
            cursor: *self,
            value: Some(value),
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn span_to(&self, end: &Cursor<'_>) -> Span {
        assert!(self.src_id == end.src_id);
        Span::new(self.src_id, self.offset, end.offset)
    }

    /// Advance by `n` bytes, counting characters for the offset.
    pub fn advance(&self, n: usize) -> Self {
        let consumed = &self.rest[..n];
        let chars = consumed.chars().count();

        Self {
            src_id: self.src_id,
            rest: &self.rest[n..],
            offset: self.offset + chars,
        }
    }

    /// Advance while predicate holds on chars.
    pub fn skip_while(&self, mut pred: impl FnMut(char) -> bool) -> Self {
        let mut bytes = 0;

        for ch in self.rest.chars() {
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

            if next.offset() != self.offset() {
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
