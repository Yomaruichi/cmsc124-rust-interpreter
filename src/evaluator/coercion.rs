use crate::token::{Token, Value};
use crate::evaluator::RuntimeError;

// Extracts f64 only from Number values; booleans, strings, and nil are rejected.
pub fn coerce_number(operator: &Token, value: &Value) -> Result<f64, RuntimeError> {
    match value {
        Value::Number(n) => Ok(*n),
        Value::Nil => Err(RuntimeError::new(operator, "Nil cannot be used in arithmetic.")),
        _ => Err(RuntimeError::new(operator, "Operand must be a number.")),
    }
}
