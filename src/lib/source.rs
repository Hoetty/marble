use crate::token::Token;

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