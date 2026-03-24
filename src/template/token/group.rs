use crate::template::token::{Delim, Stream};
use crate::template::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    span: Span,
    delim: Delim,
    content: Stream,
}

impl Group {
    pub fn new(span: Span, delim: Delim, content: Stream) -> Self {
        Self {
            span,
            delim,
            content,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn content(&self) -> &Stream {
        &self.content
    }

    pub fn into_content(self) -> Stream {
        self.content
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.delim.open(),
            &self.content,
            self.delim.close()
        )
    }
}
