use crate::template::source::SourceId;

use super::Source;

#[derive(Debug, Clone, Default)]
pub struct SourceMap(Vec<Source>);

impl SourceMap {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, id: SourceId) -> Option<&Source> {
        self.0.iter().find(|src| src.id() == id)
    }

    pub fn get_mut(&mut self, id: SourceId) -> Option<&mut Source> {
        self.0.iter_mut().find(|src| src.id() == id)
    }

    pub fn push(&mut self, src: impl Into<String>) -> SourceId {
        let id = SourceId::from(self.0.len());
        let src = Source::new(id, src);
        self.0.push(src);
        id
    }
}
