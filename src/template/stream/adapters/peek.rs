use crate::template::{Diagnostics, Scan, Scanner};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Peek<T: Scanner> {
    pos: T,
}

impl<T: Scanner> Peek<T> {
    pub(in super::super) fn new(pos: T) -> Self {
        Self { pos }
    }

    pub fn peek<V: Scan>(&self) -> Option<V> {
        let mut cursor = *self;
        let mut diagnostics = Diagnostics::default();
        let peeked = V::scan(&mut cursor, &mut diagnostics)?;
        Some(peeked)
    }
}

impl<T: Scanner> Scanner for Peek<T> {
    fn cursor(&self) -> crate::template::Cursor<'_> {
        self.pos.cursor()
    }

    fn advance_mut(&mut self, n: usize) -> Option<&mut Self> {
        self.pos.advance_mut(n)?;
        Some(self)
    }
}
