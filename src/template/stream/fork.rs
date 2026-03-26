use crate::template::{Cursor, Diagnostic, Stream};

pub struct Fork<'a, T: Stream + Clone> {
    prev: &'a mut T,
    next: T,
}

impl<'a, T: Stream + Clone> Fork<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self {
            next: value.clone(),
            prev: value,
        }
    }

    pub fn cancel(self) -> &'a mut T {
        self.prev
    }

    pub fn merge(self) -> &'a mut T {
        self.prev.commit(self.next);
        self.prev
    }
}

impl<'a, T: Stream + Clone> Stream for Fork<'a, T> {
    type Item = T::Item;

    fn cursor(&self) -> Cursor {
        self.next.cursor()
    }

    fn buffer(&self) -> &[Self::Item] {
        self.next.buffer()
    }

    fn location(&self) -> crate::template::source::Location {
        self.next.location()
    }

    fn remaining(&self) -> usize {
        self.next.remaining()
    }

    fn commit(&mut self, next: Self) {
        self.next.commit(next.next);
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

    fn skip(&mut self) -> Option<&mut Self> {
        self.next.skip()?;
        Some(self)
    }
}
