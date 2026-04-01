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
    fn peek(self) -> Peekable<Self::Item> {
        Peekable::new(self)
    }
}

// ///
// /// A reader of some Stream
// ///
// pub trait Stream: Read {
//     fn is_eof(&self) -> bool;
//     fn location(&self) -> Location;

//     fn take(&mut self) -> Option<Self::Item>;
//     fn take_n(&mut self, n: usize) -> Option<Self::Item>;

//     fn peek(&mut self) -> Option<Self::Item>;
//     fn peek_n(&mut self, n: usize) -> Option<Self::Item>;

//     fn skip(&mut self);
//     fn skip_n(&mut self, n: usize) {
//         for _ in 0..n {
//             self.skip();
//         }
//     }
// }
