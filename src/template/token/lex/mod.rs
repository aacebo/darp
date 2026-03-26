mod buffer;

pub use buffer::*;

pub trait Scan: Sized {
    fn scan(s: &mut TokenBuffer) -> Option<Self>;
}
