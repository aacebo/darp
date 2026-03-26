mod cursor;
mod fork;

pub use cursor::*;
pub use fork::*;

use crate::template::{Diagnostic, source::Location};

pub trait Stream: Sized {
    type Item;

    fn location(&self) -> Location;
    fn cursor(&self) -> Cursor;
    fn buffer(&self) -> &[Self::Item];
    fn emit(&mut self, diagnostic: Diagnostic);
    fn commit(&mut self, next: Self);
    fn is_eof(&self) -> bool {
        self.remaining() == 0
    }

    fn fork(&mut self) -> Fork<'_, Self>
    where
        Self: Clone,
    {
        Fork::new(self)
    }

    fn remaining(&self) -> usize {
        self.cursor().len()
    }

    fn peek(&self) -> Option<Self::Item>;
    fn next(&mut self) -> Option<Self::Item>;
    fn next_n(&mut self, n: usize) -> Option<&[Self::Item]> {
        let index = self.buffer().len();

        for _ in 0..n {
            self.next()?;
        }

        Some(&self.buffer()[index..])
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

    fn skip(&mut self) -> Option<&mut Self>;
    fn skip_n(&mut self, n: usize) -> Option<&mut Self> {
        for _ in 0..n {
            self.skip()?;
        }

        Some(self)
    }

    fn skip_while(&mut self, mut pred: impl FnMut(Self::Item) -> bool) -> Option<&mut Self> {
        while pred(self.peek()?) {
            self.skip()?;
        }

        Some(self)
    }
}
