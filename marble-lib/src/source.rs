use line_index::{LineCol, LineIndex};

use crate::token::Token;

#[derive(Clone)]
pub struct Source<'a> {
    pub str: &'a str,
    pub idx: LineIndex,
}

impl<'a> Source<'a> {
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
            idx: LineIndex::new(source),
        }
    }
}
