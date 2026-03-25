use crate::template::{Diagnostic, Diagnostics, Span, lex::Cursor};

/// A lazy Buffer that scans T as you iterate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buffer {
    cursor: Cursor,
    diagnostics: Diagnostics,
}

impl Buffer {
    pub fn new(cursor: Cursor) -> Self {
        Self {
            cursor,
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn span(&self) -> Span {
        self.cursor.span()
    }

    pub fn text(&self) -> &str {
        self.cursor.text()
    }

    pub fn fork(&self) -> Fork {
        Fork::from(self.clone())
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) -> &mut Self {
        self.diagnostics.emit(diagnostic);
        self
    }

    pub fn advance(&mut self, n: usize) -> &mut Self {
        self.cursor = self.cursor.advance(n);
        self
    }

    pub fn skip_while(&mut self, pred: impl FnMut(char) -> bool) -> &mut Self {
        self.cursor = self.cursor.skip_while(pred);
        self
    }

    pub fn skip_whitespace(&mut self) -> &mut Self {
        self.cursor = self.cursor.skip_whitespace();
        self
    }

    pub fn skip_comment(&mut self) -> &mut Self {
        if let Some(next) = self.cursor.skip_comment() {
            self.cursor = next;
        }

        self
    }
}

#[derive(Debug, Clone)]
pub struct Fork {
    prev: Buffer,
    next: Buffer,
}

impl Fork {
    pub fn span(&self) -> Span {
        self.next.span()
    }

    pub fn text(&self) -> &str {
        self.next.text()
    }

    pub fn fork(self) -> Fork {
        Fork::from(self.next)
    }

    pub fn emit(mut self, diagnostic: Diagnostic) -> Self {
        self.next.emit(diagnostic);
        self
    }

    pub fn advance(mut self, n: usize) -> Self {
        self.next.advance(n);
        self
    }

    pub fn skip_while(mut self, pred: impl FnMut(char) -> bool) -> Self {
        self.next.skip_while(pred);
        self
    }

    pub fn skip_whitespace(mut self) -> Self {
        self.next.skip_whitespace();
        self
    }

    pub fn skip_comment(mut self) -> Self {
        self.next.skip_comment();
        self
    }

    pub fn commit(mut self) -> Buffer {
        self.prev.cursor = self.next.cursor;
        self.prev.diagnostics.extend(self.next.diagnostics);
        self.prev
    }
}

impl From<Buffer> for Fork {
    fn from(prev: Buffer) -> Self {
        let cursor = prev.cursor;

        Self {
            prev,
            next: Buffer {
                cursor,
                diagnostics: Diagnostics::default(),
            }
        }
    }
}
