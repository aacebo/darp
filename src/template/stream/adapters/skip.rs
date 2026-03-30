use crate::template::Scanner;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Skip<T: Scanner> {
    pos: T,
    n: usize,
}

impl<T: Scanner> Skip<T> {
    pub(in super::super) fn new(pos: T, n: usize) -> Self {
        Self { pos, n }
    }
}

impl<T: Scanner> Scanner for Skip<T> {
    fn cursor(&self) -> crate::template::Cursor<'_> {
        self.pos.cursor()
    }

    fn advance_mut(&mut self, n: usize) -> Option<&mut Self> {
        self.pos.advance_mut(n)?;
        Some(self)
    }
}
