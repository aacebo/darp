mod buffer;
mod cursor;
mod fork;

pub use buffer::*;
pub use cursor::*;
pub use fork::*;

use std::marker::PhantomData;

use crate::template::{Diagnostic, Diagnostics};

pub trait Scan: Sized {
    fn scan<S: Scanner<Item = Self>>(s: &mut S) -> Option<Self>;
}

pub trait Scanner: Sized {
    type Item: Scan;

    fn cursor(&self) -> Cursor;
    fn emit(&mut self, diagnostic: Diagnostic);
    fn merge(&mut self, other: Self);
    fn is_eof(&self) -> bool {
        self.remaining() == 0
    }

    fn remaining(&self) -> usize {
        self.cursor().len()
    }

    fn peek(&self) -> Option<Self::Item>;
    fn next(&mut self) -> Option<Self::Item>;
    fn next_n(&mut self, n: usize) -> Option<Vec<Self::Item>> {
        let mut items = vec![];

        for _ in 0..n {
            items.push(self.next()?);
        }

        Some(items)
    }

    fn next_if(&mut self, mut pred: impl FnMut(Self::Item) -> bool) -> Option<Self::Item> {
        if pred(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    fn next_while(&mut self, mut pred: impl FnMut(Self::Item) -> bool) -> Option<Self::Item> {
        let mut last: Option<Self::Item> = None;

        while pred(self.peek()?) {
            last = self.next();
        }

        last
    }
}

#[derive(Clone)]
pub struct Stream<T: Scan + Clone> {
    cursor: Cursor,
    diagnostics: Diagnostics,
    __data__: PhantomData<T>,
}

impl<T: Scan + Clone> Stream<T> {
    pub fn new(cursor: Cursor) -> Self {
        Self {
            cursor,
            diagnostics: Diagnostics::default(),
            __data__: PhantomData,
        }
    }
}

impl<T: Scan + Clone> Scanner for Stream<T> {
    type Item = T;

    fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.emit(diagnostic);
    }

    fn merge(&mut self, other: Self) {
        self.cursor = other.cursor;
        self.diagnostics.extend(other.diagnostics);
    }

    fn peek(&self) -> Option<Self::Item> {
        let mut copy = self.clone();
        T::scan(&mut copy)
    }

    fn next(&mut self) -> Option<Self::Item> {
        T::scan(self)
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::template::source::{Source, SourceId};

    /// Minimal Scan impl for testing: scans a single ASCII char.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Char(pub char);

    impl Scan for Char {
        fn scan<S: Scanner<Item = Self>>(s: &mut S) -> Option<Self> {
            let cursor = s.cursor();
            if cursor.is_empty() {
                return None;
            }
            let ch = cursor.as_str().chars().next()?;
            // SAFETY: In practice, Scan::scan is always called with S = Stream<Self>
            // because Buffer/Fork delegate next() to the inner Stream.
            let stream: &mut Stream<Char> = unsafe { &mut *(s as *mut S as *mut Stream<Char>) };
            stream.cursor = cursor.next();
            Some(Char(ch))
        }
    }

    #[test]
    fn next_consumes() {
        let src = Source::new(SourceId::from(0u32), "abc");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        assert_eq!(s.next(), Some(Char('a')));
        assert_eq!(s.next(), Some(Char('b')));
        assert_eq!(s.next(), Some(Char('c')));
        assert_eq!(s.next(), None);
    }

    #[test]
    fn peek_does_not_consume() {
        let src = Source::new(SourceId::from(0u32), "xy");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        assert_eq!(s.peek(), Some(Char('x')));
        assert_eq!(s.peek(), Some(Char('x')));
        assert_eq!(s.next(), Some(Char('x')));
        assert_eq!(s.peek(), Some(Char('y')));
    }

    #[test]
    fn remaining_and_eof() {
        let src = Source::new(SourceId::from(0u32), "ab");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        assert_eq!(s.remaining(), 2);
        assert!(!s.is_eof());
        s.next();
        s.next();
        assert_eq!(s.remaining(), 0);
        assert!(s.is_eof());
    }

    #[test]
    fn next_n() {
        let src = Source::new(SourceId::from(0u32), "abcd");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        let items = s.next_n(3).unwrap();

        assert_eq!(items, vec![Char('a'), Char('b'), Char('c')]);
        assert_eq!(s.remaining(), 1);
    }

    #[test]
    fn next_n_insufficient() {
        let src = Source::new(SourceId::from(0u32), "a");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));

        assert!(s.next_n(3).is_none());
    }

    #[test]
    fn next_if() {
        let src = Source::new(SourceId::from(0u32), "ab");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));

        assert_eq!(s.next_if(|c| c.0 == 'a'), Some(Char('a')));
        assert_eq!(s.next_if(|c| c.0 == 'a'), None); // 'b' doesn't match
        assert_eq!(s.remaining(), 1); // 'b' not consumed
    }

    #[test]
    fn next_while() {
        let src = Source::new(SourceId::from(0u32), "aaab");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        let last = s.next_while(|c| c.0 == 'a');

        assert_eq!(last, Some(Char('a')));
        assert_eq!(s.remaining(), 1);
        assert_eq!(s.peek(), Some(Char('b')));
    }

    #[test]
    fn emit_collects_diagnostics() {
        let src = Source::new(SourceId::from(0u32), "x");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        let span = s.cursor().span();
        s.emit(Diagnostic::new(span, crate::template::Code::UNEXPECTED_TOKEN).build());
    }
}
