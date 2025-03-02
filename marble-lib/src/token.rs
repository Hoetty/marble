#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    String(bool), Number(f64), Identifier,

    Fn, Of, 
    Do, End, 
    Let, Be, In,

    Then,

    Comment, Eof
}