use crate::template::*;

pub trait Buffered: Scanner {
    fn buffered(self) -> Buffer<Self>;
}

impl<T: Scanner> Buffered for T
where
    T::Item: Clone,
{
    fn buffered(self) -> Buffer<Self> {
        Buffer::of(self)
    }
}

/// A lazy Buffer that scans T as you iterate
#[derive(Debug, Clone)]
pub struct Buffer<T: Scanner> {
    inner: T,
    buffer: Vec<T::Item>,
}

impl<T: Scanner> Buffer<T>
where
    T::Item: Clone,
{
    pub fn of(inner: T) -> Self {
        Self {
            inner,
            buffer: vec![],
        }
    }

    pub fn skip(&mut self) -> Option<&mut Self> {
        self.inner.next()?;
        Some(self)
    }

    pub fn skip_n(&mut self, n: usize) -> Option<&mut Self> {
        for _ in 0..n {
            self.skip()?;
        }

        Some(self)
    }

    pub fn skip_while(&mut self, mut pred: impl FnMut(T::Item) -> bool) -> Option<&mut Self> {
        while pred(self.peek()?) {
            self.skip()?;
        }

        Some(self)
    }

    pub fn unzip(self) -> (T, Vec<T::Item>) {
        (self.inner, self.buffer)
    }
}

impl<T: Scanner> Scanner for Buffer<T>
where
    T::Item: Clone,
{
    type Item = T::Item;

    fn cursor(&self) -> Cursor {
        self.inner.cursor()
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.inner.emit(diagnostic);
    }

    fn merge(&mut self, other: Self) {
        self.inner.merge(other.inner);
        self.buffer.extend(other.buffer);
    }

    fn peek(&self) -> Option<Self::Item> {
        self.inner.peek()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next()?;
        self.buffer.push(item.clone());
        Some(item)
    }
}

impl<T: Scanner> From<T> for Buffer<T>
where
    T::Item: Clone,
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::source::{Source, SourceId};
    use crate::template::stream::tests::Char;

    #[test]
    fn buffer_collects_items() {
        let src = Source::new(SourceId::from(0u32), "abc");
        let stream = Stream::<Char>::new(Cursor::from_src(&src));
        let mut buf = stream.buffered();

        buf.next();
        buf.next();

        assert_eq!(&*buf, &[Char('a'), Char('b')]);
    }

    #[test]
    fn buffer_unzip() {
        let src = Source::new(SourceId::from(0u32), "xy");
        let stream = Stream::<Char>::new(Cursor::from_src(&src));
        let mut buf = Buffer::of(stream);

        buf.next();
        buf.next();

        let (_, items) = buf.unzip();
        assert_eq!(items, vec![Char('x'), Char('y')]);
    }

    #[test]
    fn buffer_peek_does_not_buffer() {
        let src = Source::new(SourceId::from(0u32), "ab");
        let stream = Stream::<Char>::new(Cursor::from_src(&src));
        let buf = stream.buffered();

        assert_eq!(buf.peek(), Some(Char('a')));
        assert!(buf.is_empty()); // deref slice is empty, nothing consumed
    }

    #[test]
    fn buffer_deref_slice() {
        let src = Source::new(SourceId::from(0u32), "abc");
        let stream = Stream::<Char>::new(Cursor::from_src(&src));
        let mut buf = stream.buffered();

        buf.next();
        buf.next();
        buf.next();

        assert_eq!(buf.remaining(), 0);
        assert_eq!((&*buf).len(), 3); // buffered items via Deref
    }
}
