use std::collections::HashMap;

use crate::{expr::IdentRef, token::Token};

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

impl <'a> IdentifierTable<'a> {

    pub fn reference(&mut self, key: &'a str) -> IdentRef {
        let new_value = self.identifiers.len();
        *self.identifiers.entry(&key).or_insert_with(|| {
            self.backwards.push(key);
            new_value
        })
    }

    pub fn name(&self, ident: IdentRef) -> &'a str {
        self.backwards[ident]
    }

    pub fn new() -> Self {
        Self::default()
    }
}