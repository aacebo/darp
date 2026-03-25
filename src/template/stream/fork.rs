use crate::template::{Diagnostic, Span, Stream};

#[derive(Clone)]
pub struct Fork<T: Stream> {
    prev: *const T,
    next: T,
}

impl<T: Stream + Clone> Fork<T> {
    pub(crate) fn new(value: &T) -> Self {
        Self {
            prev: std::ptr::from_ref(value),
            next: value.clone(),
        }
    }
}

impl<T: Stream> Fork<T> {
    pub fn span(&self) -> Span {
        self.next.span()
    }

    pub fn fork(self) -> Fork<Self> {
        Fork::new(self)
    }

    pub fn emit(mut self, diagnostic: Diagnostic) -> Self {
        self.next.emit(diagnostic);
        self
    }

    pub fn advance(mut self, n: usize) -> Self {
        self.next.advance(n);
        self
    }

    // pub fn skip_while(mut self, pred: impl FnMut(char) -> bool) -> Self {
    //     self.next.skip_while(pred);
    //     self
    // }

    // pub fn skip_whitespace(mut self) -> Self {
    //     self.next.skip_whitespace();
    //     self
    // }

    // pub fn skip_comment(mut self) -> Self {
    //     self.next.skip_comment();
    //     self
    // }

    pub fn commit(mut self) -> T {
        self.prev.cursor = self.next.cursor;
        self.prev.diagnostics.extend(self.next.diagnostics);
        self.prev
    }
}

impl<T: Stream> Stream for Fork<T> {
    fn span(&self) -> Span {
        self.next.span()
    }

    fn location(&self) -> crate::template::source::Location {
        self.next.location()
    }

    fn emit(&mut self, diagnostic: Diagnostic) {
        self.next.emit(diagnostic);
    }
}
