use crate::token::{Token, Value};

/// Root of the tree: statements in source order.
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Variable { name: Token, initializer: Option<Expr> },
}

pub fn print_statement(statement: &Stmt) -> String {
    match statement {
        Stmt::Expression(expr) => print(expr),
        Stmt::Print(expr) => format!("(ipakita {})", print(expr)),
        Stmt::Variable { name, initializer } => {
            match initializer {
                Some(expr) => format!("(itakda {} {})", name.lexeme, print(expr)),
                None => format!("(itakda {})", name.lexeme),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(Token),
    Assign { name: Token, value: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping(Box<Expr>),
}

pub fn print(expr: &Expr) -> String {
    match expr {
        Expr::Variable(name) => name.lexeme.clone(),
        Expr::Assign { name, value } => format!("(= {} {})", name.lexeme, print(value)),
        Expr::Literal(value) => match value {
            Value::String(s) => format!("{:?}", s),
            other => other.to_string(),
        },
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
