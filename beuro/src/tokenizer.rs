use crate::token::{Token, TokenType};

pub struct Tokenizer {
    pub src_code: Vec<char>,
    idx: usize,
}

impl Tokenizer {
    pub fn new(src_code: Vec<char>) -> Self {
        Self {
            src_code: src_code,
            idx: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let chars: &Vec<char> = &self.src_code;
        let mut buf = String::new();
        let mut i: usize = 0;

        while i < chars.len() {
            let c: char = chars[i];
            //Check if it is keyword
            if c.is_alphabetic() || c == '_' {
                buf.push(c);
                i += 1;
                while i < chars.len() && chars[i].is_alphanumeric() {
                    buf.push(chars[i]);
                    i += 1;
                }
                i -= 1;

                if buf == "conclude" {
                    tokens.push(Token {
                        token_type: TokenType::Return,
                        token_value: None,
                    });
                    buf.clear();
                } else {
                    println!("Some Error Occurred when checking Keyword!");
                    std::process::exit(1);
                }
            }
            //Check if it is Integer Literal
            else if c.is_digit(10) {
                buf.push(c);
                i += 1;

                while i < chars.len() && chars[i].is_digit(10) {
                    buf.push(chars[i]);
                    i += 1;
                }

                i -= 1;

                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    token_value: Some(std::mem::take(&mut buf)),
                });
            }
            //Check for Semi-Colon
            else if c == ';' {
                tokens.push(Token {
                    token_type: TokenType::Semi,
                    token_value: None,
                });
            }
            //Skip Spaces
            else if c.is_whitespace() {
            } else {
                println!("An Error Occurred during tokenization!");
            }

            i += 1;
        }

        tokens
    }

    fn consume_char(&mut self) -> char {
        let result = self.src_code[self.idx];
        self.idx += 1;

        result
    }
}
