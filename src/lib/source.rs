use std::collections::HashMap;

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
    pub fn new(source: &'a str) -> Self {
        Self {
            str: source
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IdentifierTable<'a> {
    identifiers: HashMap<&'a str, IdentRef>,
    backwards: Vec<&'a str>,
}

type IdentifierResult = Result<usize, Error>;

impl <'a> IdentifierTable<'a> {

    pub fn push(&mut self, key: &'a str) -> IdentifierResult {
        if self.is_defined(key) {
            Err(Error::IdentifierIsAlreadyDefined(key.to_string()))
        } else {
            let depth = self.identifiers.len();
            self.identifiers.insert(key, depth);
            self.backwards.push(key);
            Ok(depth)
        }
    }

    pub fn distance_from_root(&self, key: &'a str) -> IdentifierResult {
        self.identifiers.get(&key).copied().ok_or_else(|| Error::IdentifierIsNotDefined(key.to_string()))
    }

    pub fn distance_from_top(&self, key: &'a str) -> IdentifierResult {
        self.distance_from_root(key).map(|distance| self.backwards.len() - 1 - distance)
    }

    pub fn pop(&mut self) {
        let key = self.backwards.pop().expect("Popped entire name table");
        self.identifiers.remove(&key);
    }

    pub fn name(&self, ident: IdentRef) -> &'a str {
        self.backwards[ident]
    }

    pub fn is_defined(&self, key: &'a str) -> bool {
        self.identifiers.contains_key(key)
    }

    pub fn new() -> Self {
        Self::default()
    }
}