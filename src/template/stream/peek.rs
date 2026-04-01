use crate::template::Read;

pub struct Peekable<T: Read> {
    inner: T,
}

impl<T: Read> Peekable<T> {
    pub(super) fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn take(&mut self) -> Vec<T::Item> {
        let mut items = vec![];

        for _ in 0..self.n {
            if let Some(v) = self.inner.read() {
                items.push(v);
            } else {
                break;
            }
        }

        items
    }
}

impl<T: Read> Read for Peekable<T> {
    type Item = T::Item;

    fn read(&mut self) -> Option<Self::Item> {
        self.inner.read()
    }
}
