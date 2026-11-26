use crate::token::{
    Token,
    TokenType::{IntLit, Return, Semi},
};

pub struct Tokenizer {
    pub src_code: Vec<char>,
    idx: usize,
}

impl Tokenizer {
    pub fn new(src: Vec<char>) -> Self {
        Self {
            src_code: src,
            idx: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buf = String::new();

        while self.look_ahead(1).is_some() {
            //Check for Keyword
            if self.look_ahead(1).unwrap().is_alphabetic() || self.look_ahead(1).unwrap() == '_' {
                buf.push(self.consume_char());

                while self.look_ahead(1).is_some() && self.look_ahead(1).unwrap().is_alphanumeric()
                {
                    buf.push(self.consume_char());
                }

                if buf == "conclude" {
                    tokens.push(Token {
                        token_type: Return,
                        token_value: None,
                    });

                    buf.clear();
                    continue;
                } else {
                    println!("There was an error detecting the keyword!");
                }
            }
            //Checking for Integer Literal
            else if self.look_ahead(1).unwrap().is_ascii_digit() {
                buf.push(self.consume_char());

                while self.look_ahead(1).is_some() && self.look_ahead(1).unwrap().is_ascii_digit() {
                    buf.push(self.consume_char());
                }

                tokens.push(Token {
                    token_type: IntLit,
                    token_value: Some(std::mem::take(&mut buf)),
                });
                continue;
            }
            //Check for Semi-Colon
            else if self.look_ahead(1).unwrap() == ';' {
                buf.push(self.consume_char());
                tokens.push(Token {
                    token_type: Semi,
                    token_value: None,
                });
                continue;
            }
            //Check for Space
            else if self.look_ahead(1).unwrap().is_whitespace() {
                self.consume_char();
                continue;
            } else {
                println!("Tokenization Error Occurred!");
            }
        }

        self.idx = 0;
        tokens
    }

    fn consume_char(&mut self) -> char {
        let result = self.src_code[self.idx];
        self.idx += 1;

        result
    }

    fn look_ahead(&mut self, ahead: usize) -> Option<char> {
        //Keep an eye on this, not sure about the >=
        if self.idx + ahead >= self.src_code.len() {
            None
        } else {
            Some(self.src_code[self.idx])
        }
    }
}
