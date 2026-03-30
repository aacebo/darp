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

    // fn skip(&mut self) -> Option<&mut Self>;
    // fn skip_n(&mut self, n: usize) -> Option<&mut Self> {
    //     for _ in 0..n {
    //         self.skip()?;
    //     }

    //     Some(self)
    // }

    // fn skip_while(&mut self, mut pred: impl FnMut(Self::Item) -> bool) -> Option<&mut Self> {
    //     while pred(self.peek()?) {
    //         self.skip()?;
    //     }

    //     Some(self)
    // }
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
