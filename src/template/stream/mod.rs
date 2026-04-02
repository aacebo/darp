mod cursor;
mod peek;

pub use cursor::*;
pub use peek::*;

pub trait Scan: Sized {
    type Scanner: Read;

    fn scan(s: &mut Self::Scanner) -> Option<Self>;
}

pub trait Peek: Sized {
    type Scanner: Read;

    fn peek(s: &Self::Scanner) -> Option<Self>;
}

pub trait Read: Sized {
    type Item;

    fn read(&mut self) -> Option<Self::Item>;

    // Provided

    fn peekable(self) -> Peekable<Self> {
        Peekable::new(self)
    }
}

