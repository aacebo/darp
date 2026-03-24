/// 0 indexed location
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Location {
    index: usize,
    line: usize,
    column: usize,
}

impl Location {
    pub const fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column,
        }
    }

    /// the byte index
    pub const fn index(&self) -> usize {
        self.index
    }

    /// the line number
    pub const fn line(&self) -> usize {
        self.line
    }

    /// the column number
    pub const fn column(&self) -> usize {
        self.column
    }
}
