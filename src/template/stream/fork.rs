use crate::template::{Cursor, Diagnostic, Scanner};

pub trait Fork: Scanner {
    fn fork(&mut self) -> Forked<'_, Self>;
}

impl<T: Scanner + Clone> Fork for T {
    fn fork(&mut self) -> Forked<'_, Self> {
        Forked::of(self, self.clone())
    }
}

pub struct Forked<'a, T: Scanner> {
    inner: &'a mut T,
    next: T,
}

impl<'a, T: Scanner> Forked<'a, T> {
    pub fn of(inner: &'a mut T, next: T) -> Self {
        Self { inner, next }
    }

    pub fn cancel(self) -> &'a mut T {
        self.inner
    }

    pub fn merge(self) -> &'a mut T {
        self.inner.merge(self.next);
        self.inner
    }
}

impl<'a, T: Scanner> Scanner for Forked<'a, T> {
    type Item = T::Item;

    fn cursor(&self) -> Cursor {
        self.next.cursor()
    }

    fn merge(&mut self, other: Self) {
        self.next.merge(other.next);
    }

    fn remaining(&self) -> usize {
        self.next.remaining()
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.next.emit(diagnostic);
    }

    fn peek(&self) -> Option<Self::Item> {
        self.next.peek()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::source::{Source, SourceId};
    use crate::template::stream::Stream;
    use crate::template::stream::tests::Char;

    #[test]
    fn cancel_preserves_original() {
        let src = Source::new(SourceId::from(0u32), "abcd");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));

        s.next(); // consume 'a'

        let mut forked = s.fork();

        forked.next(); // consume 'b' in fork
        forked.next(); // consume 'c' in fork

        let original = forked.cancel();

        assert_eq!(original.remaining(), 3); // still at 'b'
        assert_eq!(original.peek(), Some(Char('b')));
    }

    #[test]
    fn merge_applies_changes() {
        let src = Source::new(SourceId::from(0u32), "abcd");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        let mut forked = s.fork();

        forked.next(); // 'a'
        forked.next(); // 'b'

        let original = forked.merge();

        assert_eq!(original.remaining(), 2);
        assert_eq!(original.peek(), Some(Char('c')));
    }

    #[test]
    fn peek_sees_fork_state() {
        let src = Source::new(SourceId::from(0u32), "xyz");
        let mut s = Stream::<Char>::new(Cursor::from_src(&src));
        let mut forked = s.fork();

        forked.next(); // consume 'x'
        assert_eq!(forked.peek(), Some(Char('y')));

        // original unchanged
        forked.cancel();
        assert_eq!(s.peek(), Some(Char('x')));
    }
}
