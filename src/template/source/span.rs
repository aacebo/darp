use super::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Span {
    /// the source id this span belongs to
    src_id: SourceId,

    /// the start char index (inclusive).
    start: usize,

    /// the end char index (exclusive).
    end: usize,
}

impl Span {
    pub const fn new(src_id: SourceId, start: usize, end: usize) -> Self {
        Self { src_id, start, end }
    }

    pub fn src_id(&self) -> SourceId {
        self.src_id
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.src_id == other.src_id && self.start >= other.start && self.end <= other.end
    }

    pub fn join(self, other: Self) -> Self {
        assert!(self.src_id == other.src_id);

        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };

        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };

        Self {
            src_id: self.src_id,
            start,
            end,
        }
    }
}
