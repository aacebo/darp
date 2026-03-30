use crate::template::*;

pub trait Buffered: Scanner {
    fn buffered(self) -> Buffer<Self>;
}

impl<T: Scanner> Buffered for T {
    fn buffered(self) -> Buffer<Self> {
        Buffer::of(self)
    }
}

/// A lazy Buffer that scans T as you iterate
#[derive(Debug, Clone)]
pub struct Buffer<T: Scanner> {
    stream: T,
    buffer: Vec<T::Item>,
}

impl<T: Scanner> Buffer<T> {
    pub fn of(stream: T) -> Self {
        Self {
            stream,
            buffer: vec![],
        }
    }

    pub fn unzip(self) -> (T, Vec<T::Item>) {
        (self.stream, self.buffer)
    }
}

impl<T: Scanner> Scanner for Buffer<T> {
    type Item = T::Item;

    fn cursor(&self) -> Cursor {
        self.stream.cursor()
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.stream.emit(diagnostic);
    }

    fn merge(&mut self, other: Self) {
        self.stream.merge(other.stream);
        self.buffer.extend(other.buffer);
    }

    fn peek(&self) -> Option<Self::Item> {
        self.stream.peek()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.next()
    }
}

impl<T: Scanner> From<T> for Buffer<T> {
    fn from(value: T) -> Self {
        Self::of(value)
    }
}

impl<T: Scanner> std::ops::Deref for Buffer<T> {
    type Target = [T::Item];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
