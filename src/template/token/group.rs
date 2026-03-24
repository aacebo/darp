#![allow(unused)]

use crate::template::*;

#[derive(Debug, Clone)]
pub struct Group {
    delim: super::Delim,
    span: Span,
    content: super::Stream,
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.content)
    }
}
