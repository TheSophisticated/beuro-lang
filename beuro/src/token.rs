#[derive(PartialEq, Clone)]
pub enum TokenType {
    Return,
    IntLit,
    Semi,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: Option<String>,
}
