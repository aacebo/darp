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
