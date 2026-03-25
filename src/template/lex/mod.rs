mod cursor;
mod error;

pub use cursor::*;
pub use error::*;

use crate::template::{Diagnostic, Diagnostics};

pub trait Scan: Sized {
    fn scan(cursor: Cursor<'_>) -> ScanResult<'_, Self>;
}

pub struct ScanResult<'a, T> {
    cursor: Cursor<'a>,
    value: Option<T>,
    diagnostics: Diagnostics,
}

impl<'a, T> ScanResult<'a, T> {
    pub fn emit(mut self, diagnostic: Diagnostic) -> Self {
        self.diagnostics.emit(diagnostic);
        self
    }

    pub fn build(self) -> (Cursor<'a>, Option<T>, Diagnostics) {
        (self.cursor, self.value, self.diagnostics)
    }
}

impl<'a, T> From<Cursor<'a>> for ScanResult<'a, T> {
    fn from(cursor: Cursor<'a>) -> Self {
        Self {
            cursor,
            value: None,
            diagnostics: Diagnostics::default(),
        }
    }
}
