mod cursor;
mod buffer;

pub use cursor::*;
pub use buffer::*;

use crate::template::{Stream};

pub trait Scan: Sized {
    fn scan(s: &mut impl Stream) -> Option<Self>;
}
