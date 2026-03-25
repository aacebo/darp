mod fork;

pub use fork::*;

use crate::template::{Diagnostic, Span, source::Location};

pub trait Stream: Sized {
    type Item;

    fn span(&self) -> Span;
    fn location(&self) -> Location;
    fn remaining(&self) -> usize;
    fn emit(&mut self, diagnostic: Diagnostic) -> &mut Self;
    fn fork(&self) -> Fork<Self>;

    fn peek_n(&self, n: usize) -> Option<Self::Item>;
    fn peek(&self) -> Option<Self::Item> {
        self.peek_n(1)
    }

    fn next_n(&mut self, n: usize) -> Option<Self::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_n(1)
    }

    fn skip_n(&mut self, n: usize) -> Option<&mut Self>;
    fn skip_while(&mut self, pred: impl FnMut(Self::Item) -> bool) -> Option<&mut Self>;
    fn skip(&mut self) -> Option<&mut Self> {
        self.skip_n(1)
    }
}
