use line_index::TextRange;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub range: TextRange
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum TokenType {
    String(bool), Number(f64), Identifier,

    Fn, Of, 
    Do, End, 
    Let, Be, In,

    Then,

    Comment, Eof,

    #[default]
    Generated
}