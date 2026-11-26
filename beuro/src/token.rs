#[derive(PartialEq)]
pub enum TokenType {
    Return,
    IntLit,
    Semi,
}

pub struct Token {
    pub token_type: TokenType,
    pub token_value: Option<String>,
}
