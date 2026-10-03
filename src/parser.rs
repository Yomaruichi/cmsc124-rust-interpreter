use crate::ast::Expr;
use crate::token::{Token, TokenType, Value};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}