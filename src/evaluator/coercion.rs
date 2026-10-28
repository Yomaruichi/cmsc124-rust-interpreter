use crate::token::{Token, Value};
use crate::evaluator::RuntimeError;

// Converts numbers unchanged and booleans to 1 or 0; rejects string and nil values.
pub fn coerce_number(operator: &Token, value: &Value) -> Result<f64, RuntimeError> {
    match value {
        Value::Number(n) => Ok(*n),
        Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::Nil => Err(RuntimeError::new(operator, "Nil cannot be used in arithmetic.")),
        Value::String(_) => Err(RuntimeError::new(operator, "Operand must be a number or boolean.")),
    }
}
