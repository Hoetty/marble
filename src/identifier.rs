use crate::error::Error;

#[derive(Clone, Debug, Default)]
pub struct IdentifierTable<'a> {
    identifiers: Vec<&'a str>,
}

pub type IdentRef = usize;
type IdentifierResult = Result<usize, Error>;

impl<'a> IdentifierTable<'a> {
    pub fn push(&mut self, key: &'a str) -> usize {
        let depth = self.identifiers.len();
        self.identifiers.push(key);
        depth
    }

    pub fn distance_from_root(&self, key: &'a str) -> IdentifierResult {
        self.identifiers
            .iter()
            .rev()
            .position(|ident| *ident == key)
            .ok_or_else(|| Error::IdentifierIsNotDefined(key.to_string()))
            .map(|idx| self.identifiers.len() - 1 - idx)
    }

    pub fn distance_from_top(&self, key: &'a str) -> IdentifierResult {
        self.distance_from_root(key)
            .map(|distance| self.identifiers.len() - 1 - distance)
    }

    pub fn pop(&mut self) {
        self.identifiers.pop().expect("Popped entire name table");
    }

    pub fn name(&self, ident: IdentRef) -> &'a str {
        self.identifiers[ident]
    }

    pub fn new() -> Self {
        Self::default()
    }
}
