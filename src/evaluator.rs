mod coercion;

use crate::environment::Environment;
use crate::ast::{Expr, Program, Stmt};
use crate::token::{Token, TokenType, Value};
use crate::evaluator::coercion::coerce_number;
 

 
// Keeps the operator token so the error can report a line number.
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

// create runtime error constructor
impl RuntimeError {
    pub(crate) fn new(token: &Token, message: &str) -> Self {
        RuntimeError { token: token.clone(), message: message.to_string() }
    }
}

// just prints out where in the code was the error; line and message included
pub fn report_runtime_error(err: &RuntimeError) {
    eprintln!("[line {}] Runtime error: {}", err.token.line, err.message);
}
 
 

// File execution prints explicit output. REPL and --eval also echo expressions.
pub fn interpret(program: &Program, environment: &mut Environment, echo: bool) -> Result<(), RuntimeError> {
    execute_statements(&program.statements, environment, echo)
}

fn execute_statements(statements: &[Stmt], environment: &mut Environment, echo: bool) -> Result<(), RuntimeError> {
    for statement in statements {
        match statement {
            Stmt::Block(statements) => {
                let enclosing = std::mem::replace(environment, Environment::new());
                *environment = Environment::enclosed(enclosing);
                // Save the result before restoring so an error cannot skip scope exit.
                let result = execute_statements(statements, environment, false);
                let block = std::mem::replace(environment, Environment::new());
                *environment = block.into_enclosing();
                result?;
            }
            Stmt::Variable { name, initializer } => {
                let value = match initializer {
                    Some(expr) => evaluate(expr, environment)?,
                    None => Value::Nil,
                };
                environment.define(name, value);
            }
            Stmt::Print(expr) => {
                let value = evaluate(expr, environment)?;
                println!("{}", stringify(&value));
            }
            Stmt::Expression(expr) => {
                let value = evaluate(expr, environment)?;
                if echo {
                    println!("{}", stringify(&value));
                }
            }
        }
    }
    Ok(())
}

// runtime value -> output text. Unlike Value's Display (which keeps "5.0"
// for the tokenizer), whole numbers print as "5" here.
pub fn stringify(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::Boolean(n) => n.to_string(),
        Value::Nil => "nil".to_string(),
        other => other.to_string(),
    }
}

// Evaluates an expression tree, preserving literal/grouped values and propagating runtime errors.
// Unary ! accepts any Value through truthiness; unary minus accepts numbers only.
pub fn evaluate(expr: &Expr, environment: &mut Environment) -> Result<Value, RuntimeError> {

    // evaluate each enum type
    // start the backbone with literals first
    match expr {
        Expr::Variable(name) => environment.get(name),
        Expr::Assign { name, value } => {
            let value = evaluate(value, environment)?;
            environment.assign(name, value.clone())?;
            Ok(value)
        }

        Expr::Literal(value)=> {
            return Ok(value.clone());
        }

        Expr::Unary{operator, right} => {
            let right = evaluate(right, environment)?;
            match operator.token_type {
                TokenType::MINUS => {
                    let n = number_unary(operator, &right)?;
                    Ok(Value::Number(-n))
                }
                TokenType::NOT => Ok(Value::Boolean(!is_truthy(&right))),

                // add an error here for a unsupported operator
                _ => todo!()
            }
        }

        Expr::Binary{left, operator, right} => {
            evaluate_binary(left, operator, right, environment)
        }

        Expr::Grouping (inner) => {
            evaluate(inner, environment)
        }
    }
}
 


// Extracts a numeric operand for unary minus; every non-number produces a runtime error.
fn number_unary(operator: &Token, value: &Value) -> Result<f64, RuntimeError>{
    coerce_number(operator, value)
}


// Accepts any runtime Value; nil, false, zero (including -0.0), and empty strings are falsey.
// Nonzero numbers and nonempty strings (including whitespace) are truthy without conversion.
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
    }
}

// Evaluates left first; logical operators accept all values and skip right when left decides the result.
// Other operators evaluate both sides, then apply arithmetic, numeric ordering, or strict equality.
fn evaluate_binary(left: &Expr, operator: &Token, right: &Expr, environment: &mut Environment) -> Result<Value, RuntimeError> {
 
    let l = evaluate(left, environment)?;
    match operator.token_type {
        TokenType::AT => {
            if !is_truthy(&l) {
                return Ok(Value::Boolean(false));
            }
            return Ok(Value::Boolean(is_truthy(&evaluate(right, environment)?)));
        }
        TokenType::OKAYA => {
            if is_truthy(&l) {
                return Ok(Value::Boolean(true));
            }
            return Ok(Value::Boolean(is_truthy(&evaluate(right, environment)?)));
        }
        _ => {}
    }
    let r = evaluate(right, environment)?;
 
    match operator.token_type {
        
     
        TokenType::MINUS => {
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a - b))
        }
        TokenType::STAR => {
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a * b))
        }
        TokenType::SLASH => {
            let (a, b) = number_operands(operator, &l, &r)?;
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Division by zero."));
            }
            Ok(Value::Number(a / b))
        }

        TokenType::MODULO => {
            let (a, b) = number_operands(operator, &l, &r)?;
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Modulo by zero."));
            }
            Ok(Value::Number(a % b))
        }

        TokenType::PLUS => {
            if matches!(l, Value::Nil) || matches!(r, Value::Nil) {
                return Err(RuntimeError::new(operator, "Nil cannot be used in arithmetic."));
            }
            if matches!(l, Value::String(_)) || matches!(r, Value::String(_)) {
                return Ok(Value::String(stringify(&l) + &stringify(&r)));
            }
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a + b))
        },

        TokenType::GREATER | TokenType::GREATEREQUAL | TokenType::LESS | TokenType::LESSEQUAL => {
            if matches!(l, Value::Nil) || matches!(r, Value::Nil) {
                return Err(RuntimeError::new(operator, "Nil cannot be used in comparisons."));
            }
            let (a, b) = number_operands(operator, &l, &r)?;
            let result = match operator.token_type {
                TokenType::GREATER => a > b,
                TokenType::GREATEREQUAL => a >= b,
                TokenType::LESS => a < b,
                _ => a <= b, // This arm can only be LESSEQUAL after the outer match.
            };
            Ok(Value::Boolean(result))
        }
        // Value's derived PartialEq compares both enum variant and contained value.
        TokenType::EQUALEQUAL => Ok(Value::Boolean(l == r)),
        TokenType::NOTEQUAL => Ok(Value::Boolean(l != r)),

        (_) => todo!()
    }
}

// Extracts two numeric operands for arithmetic and ordering; no boolean conversion is performed.
// The operator token preserves the source line when either value fails the shared type check.
fn number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64), RuntimeError> {
    Ok((coerce_number(operator, left)?, coerce_number(operator, right)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    #[test]
    fn nested_runtime_error_restores_global_visibility_and_releases_block_bindings() {
        let mut environment = Environment::new();
        let mut scanner = Scanner::new_string(
            "itakda x = 1; { itakda x = 2; { itakda local = 3; 1 / 0; } }"
        );
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().ok().expect("valid syntax");

        // A nested runtime error must unwind both block environments.
        assert!(interpret(program, &mut environment, false).is_err());

        // Evaluate another input using the same environment, as the REPL does.
        let mut scanner = Scanner::new_string("x;");
        let mut parser = Parser::new(scanner.scan_tokens().clone());
        let program = parser.parse_program().ok().expect("valid syntax");
        if let Stmt::Expression(expr) = &program.statements[0] {
            assert!(matches!(evaluate(expr, &mut environment), Ok(Value::Number(1.0))));
        } else {
            panic!("expected expression statement");
        }

        let mut scanner = Scanner::new_string("local;");
        let mut parser = Parser::new(scanner.scan_tokens().clone());
        let program = parser.parse_program().ok().expect("valid syntax");
        assert!(interpret(program, &mut environment, false).is_err());
    }

    #[test]
    fn outer_assignment_survives_an_error_in_an_inner_scope() {
        let mut environment = Environment::new();
        let mut scanner = Scanner::new_string("itakda x = 1; { x = 9; 1 / 0; }");
        let mut parser = Parser::new(scanner.scan_tokens().clone());
        let program = parser.parse_program().ok().expect("valid syntax");

        // Restoring the scope keeps assignments that already changed the outer binding.
        assert!(interpret(program, &mut environment, false).is_err());
        let mut scanner = Scanner::new_string("x;");
        let mut parser = Parser::new(scanner.scan_tokens().clone());
        let program = parser.parse_program().ok().expect("valid syntax");
        if let Stmt::Expression(expr) = &program.statements[0] {
            assert!(matches!(evaluate(expr, &mut environment), Ok(Value::Number(9.0))));
        } else {
            panic!("expected expression statement");
        }
    }
}
