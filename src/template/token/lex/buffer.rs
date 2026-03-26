use crate::template::source::*;
use crate::template::*;

/// A lazy Buffer that scans T as you iterate
#[derive(Debug, Clone)]
pub struct TokenBuffer {
    src: *const Source,
    cursor: Cursor,
    buffer: Vec<Token>,
    diagnostics: Diagnostics,
}

impl TokenBuffer {
    pub fn new(src: &Source) -> Self {
        let cursor = Cursor::new(src);

        Self {
            src: std::ptr::from_ref(src),
            cursor,
            buffer: vec![],
            diagnostics: Diagnostics::default(),
        }
    }
}

impl Stream for TokenBuffer {
    type Item = Token;

    fn location(&self) -> Location {
        unsafe { self.src.read().location(self.cursor.index()) }
    }

    fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn buffer(&self) -> &[Self::Item] {
        &self.buffer
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.emit(diagnostic);
    }

    fn commit(&mut self, next: Self) {
        self.cursor = next.cursor;
        self.diagnostics.extend(next.diagnostics);
    }

    fn peek(&self) -> Option<Self::Item> {
        self.clone().next()
    }

    fn next(&mut self) -> Option<Self::Item> {
        use super::Scan;

        let token = Token::scan(self)?;
        self.buffer.push(token.clone());
        Some(token)
    }

    fn skip(&mut self) -> Option<&mut Self> {
        use super::Scan;

        Token::scan(self)?;
        Some(self)
    }
}

impl Eq for TokenBuffer {}

impl PartialEq for TokenBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.cursor == other.cursor && self.buffer == other.buffer
    }
}

impl std::hash::Hash for TokenBuffer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for token in &self.buffer {
            token.hash(state);
        }
    }
}

impl IntoIterator for TokenBuffer {
    type IntoIter = IntoIter;
    type Item = Token;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl std::fmt::Display for TokenBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.buffer {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}

pub struct IntoIter(TokenBuffer);

impl Iterator for IntoIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
