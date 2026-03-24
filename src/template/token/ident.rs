#![allow(unused)]

use crate::template::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    sym: Box<str>,
    span: Span,
    raw: bool,
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sym)
    }
}
