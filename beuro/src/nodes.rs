use crate::token::Token;

pub struct NodeExpr {
    pub int_literal: Token,
}

pub struct NodeExit {
    pub expr: NodeExpr,
}
