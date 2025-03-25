use line_index::{LineCol, LineIndex};

use crate::{error::Error, expr::IdentRef, token::Token};

#[derive(Clone)]
pub struct Source<'a> {
    pub str: &'a str,
    pub idx: LineIndex,
}

impl <'a> Source<'a> {
    #[inline]
    pub fn lexeme(&self, token: &Token) -> &'a str {
        &self.str[token.range]
    }

    pub fn start(&self, token: &Token) -> LineCol {
        self.idx.line_col(token.range.start())
    }

    pub fn end(&self, token: &Token) -> LineCol {
        self.idx.line_col(token.range.end())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.str.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.str.is_empty()
    }

    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            str: source,
            idx: LineIndex::new(source)
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IdentifierTable<'a> {
    identifiers: Vec<&'a str>
}

type IdentifierResult = Result<usize, Error>;

impl <'a> IdentifierTable<'a> {

    pub fn push(&mut self, key: &'a str) -> usize {
        let depth = self.identifiers.len();
        self.identifiers.push(key);
        depth
    }

    pub fn distance_from_root(&self, key: &'a str) -> IdentifierResult {
        self.identifiers.iter()
            .rev()
            .position(|ident| *ident == key)
            .ok_or_else(|| Error::IdentifierIsNotDefined(key.to_string()))
            .map(|idx| self.identifiers.len() - 1 - idx)
    }

    pub fn distance_from_top(&self, key: &'a str) -> IdentifierResult {
        self.distance_from_root(key).map(|distance| self.identifiers.len() - 1 - distance)
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