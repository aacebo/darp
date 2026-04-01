use crate::template::Read;

pub struct Peekable<T: Read> {
    inner: T,
    peeked: Option<Option<T::Item>>,
}

impl<T: Read> Peekable<T> {
    pub(super) fn new(inner: T) -> Self {
        Self {
            inner,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&T::Item> {
        let inner = &mut self.inner;
        self.peeked.get_or_insert_with(|| inner.read()).as_ref()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T::Item> {
        let inner = &mut self.inner;
        self.peeked.get_or_insert_with(|| inner.read()).as_mut()
    }
}

impl<T: Read> Read for Peekable<T> {
    type Item = T::Item;

    fn read(&mut self) -> Option<Self::Item> {
        self.inner.read()
    }
}
