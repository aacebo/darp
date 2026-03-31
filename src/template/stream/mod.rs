mod cursor;

pub use cursor::*;

use crate::template::source::Location;

pub trait Scan: Sized {
    type Scanner: Stream;

    fn scan(s: &mut Self::Scanner) -> Option<Self>;
}

pub trait Peek: Sized {
    type Scanner: Stream;

    fn peek(s: &Self::Scanner) -> Option<Self>;
}

pub trait Read {
    type Item;

    fn read(&mut self) -> Option<Self::Item>;
}

///
/// A reader of some Stream
///
pub trait Stream: Read {
    fn is_eof(&self) -> bool;
    fn location(&self) -> Location;

    fn take(&mut self) -> Option<Self::Item>;
    fn take_n(&mut self, n: usize) -> Option<Self::Item>;

    fn peek(&mut self) -> Option<Self::Item>;
    fn peek_n(&mut self, n: usize) -> Option<Self::Item>;

    fn skip(&mut self);
    fn skip_n(&mut self, n: usize);
}
