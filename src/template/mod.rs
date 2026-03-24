pub mod diagnostic;
pub mod lex;
pub mod parse;
pub mod source;
pub mod token;

pub use diagnostic::{Code, Diagnostic, Diagnostics, Label};
pub use lex::{LexError, Scan};
pub use source::Span;
pub use token::Token;

pub struct Output<T> {
    value: Option<T>,
    diagnostics: Diagnostics,
}

impl<T> Output<T> {
    pub fn emit(&mut self, diagnostic: Diagnostic) -> &mut Self {
        self.diagnostics.emit(diagnostic);
        self
    }
}
