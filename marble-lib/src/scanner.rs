use std::{iter::Peekable, str::Chars};

use crate::{number::deserialize, source::Source, token::{Token, TokenType}};

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    chars: Peekable<Chars<'a>>,
    source: Source<'a>
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

impl <'a> Scanner<'a> {

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.is_next_whitespace() {
            self.consume();
        }
    }

    fn next_word(&mut self) -> &str {
        self.skip_whitespace();
        self.start = self.current;

        while !self.is_at_end() && !self.is_next_whitespace() {
            self.consume();
        }

        &self.source.str[self.start..self.current]
    }

    fn next_token(&mut self) -> Token {
        if self.is_at_end() {
            self.start = self.current;
            return self.create_token(TokenType::Eof);
        }

        match self.next_word() {
            "" => self.create_token(TokenType::Eof),
            "string" => self.create_token(TokenType::String),
            "str" => self.string(),
            "com" => {
                // The comment is consumed but not returned
                self.multiline_comment();
                self.next_token()
            },
            "comment" => {
                self.comment();
                self.next_token()
            },
            word => 
                if let Some(keyword_type) = Self::check_keyword(word) {
                    self.create_token(keyword_type)
                } else if let Some(number) = Self::check_number(word) {
                    self.create_token(TokenType::Number(number))
                } else {
                    self.create_token(TokenType::Identifier)
                }
        }
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

    fn string(&mut self) -> Token {
        self.consume_until("ing");
        self.create_token(TokenType::String)
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

    fn create_token(&self, token_type: TokenType) -> Token {
        Token {
            start: self.start,
            end: self.current,
            token_type
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_next_whitespace(&mut self) -> bool {
        self.peek().is_ascii_whitespace()
    }

    fn consume_until(&mut self, target: &str) {
        while !self.is_at_end() {
            let c = self.consume();

            if !c.is_ascii_whitespace() {
                continue;
            }

            let mut test = target.chars();

            while !self.is_at_end() {
                let source_char = self.consume();
                let test_char = test.next();

                match test_char {
                    None if source_char.is_ascii_whitespace() => return,
                    None => break,
                    Some(c) if c != source_char => break,
                    _ => {}
                }
            }
        }
    }

    fn peek(&mut self) -> &char {
        self.chars.peek().unwrap()
    }

    fn consume(&mut self) -> char {
        let c = *self.peek();
        self.current += self.chars.next().unwrap().len_utf8();
        c
    }

    pub fn new(source: Source<'a>) -> Self {
        Self {
            start: 0,
            current: 0,
            source,
            chars: source.str.chars().peekable()
        }
    }
}