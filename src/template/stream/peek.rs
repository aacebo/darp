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

    pub fn read_if(&mut self, pred: impl FnOnce(&T::Item) -> bool) -> Option<T::Item> {
        match self.read() {
            Some(v) if pred(&v) => Some(v),
            other => {
                self.peeked = Some(other);
                None
            }
        }
    }

    pub fn read_while(&mut self, mut pred: impl FnMut(&T::Item) -> bool) -> usize {
        let mut count = 0;

        loop {
            if self.read_if(&mut pred).is_none() {
                break;
            }

            count += 1;
        }

        count
    }
}

impl<T: Read> Read for Peekable<T> {
    type Item = T::Item;

    fn read(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            None => self.inner.read(),
            Some(v) => v,
        }
    }
}
