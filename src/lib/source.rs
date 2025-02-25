use crate::{error::Error, expr::IdentRef, token::Token};

#[derive(Clone, Copy)]
pub struct Source<'a> {
    pub str: &'a str,
}

impl <'a> Source<'a> {
    #[inline]
    pub fn lexeme(&self, token: &Token) -> &'a str {
        &self.str[token.start..token.end]
    }

    #[inline]
    pub fn line(&self, position: usize) -> usize {
        self.str[..position].matches('\n').count() + 1
    }

    #[inline]
    pub fn line_start(&self, token: &Token) -> usize {
        self.line(token.start)
    }

    #[inline]
    pub fn line_end(&self, token: &Token) -> usize {
        self.line(token.end)
    }

    #[inline]
    pub fn column(&self, position: usize) -> usize {
        position - self.str[..position].rfind('\n').map_or(0, |v| v + 1) + 1
    }

    #[inline]
    pub fn column_start(&self, token: &Token) -> usize {
        self.column(token.start)
    }

    #[inline]
    pub fn column_end(&self, token: &Token) -> usize {
        self.column(token.end)
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
            str: source
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IdentifierTable<'a> {
    identifiers: Vec<&'a str>
}

type IdentifierResult = Result<usize, Error>;

impl <'a> IdentifierTable<'a> {

    pub fn push(&mut self, key: &'a str) -> IdentifierResult {
        let depth = self.identifiers.len();
        self.identifiers.push(key);
        Ok(depth)
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