#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    String, Bool(bool), Number(f64), Nil, Identifier,

    Fn, Do, End, If, Else, Let, Be,

    Plus, Minus, Times, Over,
    And, Or, Is, Not, Less, Greater,

    Then,

    Comment, Eof
}