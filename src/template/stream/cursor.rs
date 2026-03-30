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
            length: src.text().chars().count(),
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
        Span::new(self.src().id(), self.index, self.index + self.length)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::source::{Source, SourceId};

    fn src(text: &str) -> Source {
        Source::new(SourceId::from(0u32), text)
    }

    #[test]
    fn from_src_ascii() {
        let s = src("hello");
        let c = Cursor::from_src(&s);
        assert_eq!(c.index(), 0);
        assert_eq!(c.len(), 5);
        assert!(!c.is_empty());
    }

    #[test]
    fn from_src_multibyte() {
        let s = src("héllo");
        let c = Cursor::from_src(&s);
        assert_eq!(c.len(), 5); // 5 chars, not 6 bytes
    }

    #[test]
    fn from_src_empty() {
        let s = src("");
        let c = Cursor::from_src(&s);
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
    }

    #[test]
    fn next_advances() {
        let s = src("abc");
        let c = Cursor::from_src(&s);
        let c2 = c.next();
        assert_eq!(c2.index(), 1);
        assert_eq!(c2.len(), 2);
    }

    #[test]
    fn next_n_advances() {
        let s = src("abcde");
        let c = Cursor::from_src(&s).next_n(3);
        assert_eq!(c.index(), 3);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn next_n_clamps_at_end() {
        let s = src("ab");
        let c = Cursor::from_src(&s).next_n(10);
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
        assert_eq!(c.index(), 2);
    }

    #[test]
    fn span_covers_remaining() {
        let s = src("abcde");
        let c = Cursor::from_src(&s);
        let span = c.span();
        assert_eq!(span.start(), 0);
        assert_eq!(span.end(), 5);

        let c2 = c.next_n(2);
        let span2 = c2.span();
        assert_eq!(span2.start(), 2);
        assert_eq!(span2.end(), 5);
    }

    #[test]
    fn as_str_returns_remaining() {
        let s = src("hello");
        let c = Cursor::from_src(&s).next_n(2);
        assert_eq!(c.as_str(), "llo");
    }

    #[test]
    fn location_line_column() {
        let s = src("ab\ncd");
        let c = Cursor::from_src(&s).next_n(3); // 'c'
        let loc = c.location();
        assert_eq!(loc.line(), 1);
        assert_eq!(loc.column(), 0);
    }
}
