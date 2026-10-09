use crate::token::{Token, Value};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
}

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Literal(value) => value.to_string(),
        Expr::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, print(right))
        }
        Expr::Binary { left, operator, right } => {
            format!("({} {} {})", operator.lexeme, print(left), print(right))
        }
        Expr::Grouping(inner) => {
            format!("(group {})", print(inner))
        }
    }
}