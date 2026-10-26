use crate::ast::Expr;
use crate::token::{Token, TokenType, Value};
use crate::evaluator::{stringify};

// IMPLEMENTATION FOR DATA TYPE COERCION
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Nil,
    Boolean,
    Number,
    String,
}

fn rank(value: &Value) -> Rank {
    match value {
        Value::Nil => Rank::Nil,
        Value::Boolean(_) => Rank::Boolean,
        Value::Number(_) => Rank::Number,
        Value::String(_) => Rank::String,
    }
}

// coerce a value up to `target`
// so every function call is a widening conversion and none can fail.
fn coerce(value: &Value, target: Rank) -> Value {
    match target {
        Rank::Nil => Value::Nil,

        Rank::Boolean => match value {
            Value::Nil => Value::Boolean(false),
            other => other.clone(),
        },

        Rank::Number => match value {
            Value::Nil => Value::Number(0.0),
            Value::Boolean(b) => Value::Number(if *b { 1.0 } else { 0.0 }),
            other => other.clone(),
        },

        Rank::String => Value::String(stringify(value)),
    }
}

// Bring both operands to the higher of the two ranks.
pub fn promote(left: &Value, right: &Value) -> (Value, Value, Rank) {
    let target = rank(left).max(rank(right));
    (coerce(left, target), coerce(right, target), target)
}


