mod adapters;
mod cursor;

pub use adapters::*;
pub use cursor::*;

use crate::template::{Diagnostics, source::Location};

pub trait Scan: Sized {
    fn scan<S: Scanner>(s: &mut S, d: &mut Diagnostics) -> Option<Self>;
}

pub trait Scanner: Copy {
    fn cursor(&self) -> Cursor<'_>;
    fn advance_mut(&mut self, n: usize) -> Option<&mut Self>;

    /// Optional

    fn location(&self) -> Location {
        self.cursor().location()
    }

    fn is_eof(&self) -> bool {
        self.cursor().is_eof()
    }

    fn advance(&self, n: usize) -> Self {
        let mut next = *self;

        if let Some(v) = next.advance_mut(n) {
            next = *v;
        }

        next
    }

    // Adapters

    fn skip(self, n: usize) -> Skip<Self> {
        Skip::new(self, n)
    }

    fn peek(self) -> Peek<Self> {
        Peek::new(self)
    }
}
