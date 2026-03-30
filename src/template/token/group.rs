use crate::template::token::Delim;
use crate::template::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    span: Span,
    delim: Delim,
    // content: TokenBuffer,
}

impl Group {
    pub fn new(span: Span, delim: Delim) -> Self {
        Self {
            span,
            delim,
            // content,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    // pub fn content(&self) -> &TokenBuffer {
    //     &self.content
    // }

    // pub fn into_content(self) -> TokenBuffer {
    //     self.content
    // }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.delim.open(),
            // &self.content,
            self.delim.close()
        )
    }
}
