use crate::nodes::{NodeExit, NodeExpr};
use crate::token::{Token, TokenType};

use std::process;

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, idx: 0 }
    }

    fn parse_expression(&mut self) -> Option<NodeExpr> {
        if self.look_ahead(1).is_some()
            && self.look_ahead(1).unwrap().token_type == TokenType::IntLit
        {
            Some(NodeExpr {
                int_literal: self.consume_token(),
            })
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node: Option<NodeExit> = None;

        while self.look_ahead(1).is_some() {
            if self.look_ahead(1).unwrap().token_type == TokenType::Return {
                self.consume_token();
                let node_expr: Option<NodeExpr> = self.parse_expression();
                if node_expr.is_some() {
                    exit_node = Some(NodeExit {
                        expr: node_expr.unwrap(),
                    })
                } else {
                    println!("Invalid Expression!");
                    process::exit(1);
                }

                if !self.look_ahead(1).is_some()
                    || self.look_ahead(1).unwrap().token_type != TokenType::Semi
                {
                    println!("Invalid Expression!");
                    process::exit(1);
                } else if self.look_ahead(1).unwrap().token_type == TokenType::Semi {
                    self.consume_token();
                }
            }
        }

        self.idx = 0;
        exit_node
    }

    fn consume_token(&mut self) -> Token {
        let result = self.tokens[self.idx].clone();
        self.idx += 1;

        result
    }

    fn look_ahead(&mut self, ahead: usize) -> Option<Token> {
        if self.idx + ahead > self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.idx].clone())
        }
    }
}
