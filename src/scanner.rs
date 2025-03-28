//! Scans a marble source file or string into a stream of tokens.
//!
//! The marble grammar consists of words seperated by whitespace.
//! A word consists of any character that is not whitespace.
//! Whitespace only counts ascii whitespace.
//!
//! The following is a single valid word:
//! ++0dpp,+*äö)°
//!
//! Words are parsed as different tokens, depending on their content:
//! - 'string' -> An empty string
//! - 'str' -> Starts a string, that ends when encountering the word 'ing'
//! - 'comment' -> Starts a single line comment, ending at the next newline
//! - 'com' -> Starts a multi line comment, endig at the word 'ment'
//! - 'fn', 'of', 'do', 'end', 'let', 'be' and 'in' -> Keywords
//! - Any numeric words, like 'One', 'FortyTwo' or 'ThreePointOne' -> Number literals
//! - Every other word -> An identifier
//!
//! UTF-8 is fully supported in strings, comments and identifiers.

use std::{iter::Peekable, str::Chars};

use line_index::TextRange;

use crate::{
    number::deserialize,
    source::Source,
    token::{Token, TokenType},
};

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    chars: Peekable<Chars<'a>>,
    source: &'a Source<'a>,
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a Source<'a>) -> Self {
        Self {
            start: 0,
            current: 0,
            source,
            chars: source.str.chars().peekable(),
        }
    }

    /// Consumes the next word.
    /// A word consists of any characters, that are not ascii whitespace.
    fn next_word(&mut self) -> &str {
        self.consume_non_whitespace();
        &self.source.str[self.start..self.current]
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.create_token(TokenType::Eof);
        }

        match self.next_word() {
            "string" => self.create_token(TokenType::String(true)),
            "str" => self.string(),
            "com" => {
                // The comment is consumed but not returned
                self.multiline_comment();
                self.next_token()
            }
            "comment" => {
                self.comment();
                self.next_token()
            }
            word => {
                if let Some(keyword_type) = Self::check_keyword(word) {
                    self.create_token(keyword_type)
                } else if let Some(number) = Self::check_number(word) {
                    self.create_token(TokenType::Number(number))
                } else {
                    self.create_token(TokenType::Identifier)
                }
            }
        }
    }

    fn string(&mut self) -> Token {
        let is_terminated = self.consume_until("ing");
        self.create_token(TokenType::String(is_terminated))
    }

    fn check_keyword(word: &str) -> Option<TokenType> {
        match word {
            "fn" => Some(TokenType::Fn),
            "of" => Some(TokenType::Of),
            "do" => Some(TokenType::Do),
            "end" => Some(TokenType::End),
            "let" => Some(TokenType::Let),
            "be" => Some(TokenType::Be),
            "in" => Some(TokenType::In),
            "then" => Some(TokenType::Then),
            _ => None,
        }
    }

    fn check_number(word: &str) -> Option<f64> {
        deserialize::parse_fraction(word)
    }

    fn comment(&mut self) -> Token {
        while !self.is_at_end() && *self.peek() != '\n' {
            self.consume();
        }

        self.create_token(TokenType::Comment)
    }

    fn multiline_comment(&mut self) -> Token {
        self.consume_until("ment");
        self.create_token(TokenType::Comment)
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        Token {
            range: TextRange::new((self.start as u32).into(), (self.current as u32).into()),
            token_type,
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.is_next_whitespace() {
            self.consume();
        }
    }

    fn consume_non_whitespace(&mut self) {
        while !self.is_at_end() && !self.is_next_whitespace() {
            self.consume();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_next_whitespace(&mut self) -> bool {
        self.peek().is_ascii_whitespace()
    }

    fn consume_until(&mut self, target: &str) -> bool {
        while !self.is_at_end() {
            let c = self.consume();

            if !c.is_ascii_whitespace() {
                continue;
            }

            if self.source.str[self.current..].starts_with(target) {
                for _ in 0..target.len() {
                    self.consume();
                }

                if self.is_at_end() || self.peek().is_ascii_whitespace() {
                    return true;
                }
            }
        }

        false
    }

    fn peek(&mut self) -> &char {
        self.chars.peek().unwrap()
    }

    fn consume(&mut self) -> char {
        let c = *self.peek();
        self.current += self.chars.next().unwrap().len_utf8();
        c
    }
}
