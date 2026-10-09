use crate::ast::Expr;
use crate::token::{Token, TokenType, Value};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}

pub struct ParseError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, had_error: false }
    }

    // ---- six helpers, token-level equivalents of the scanner's ----

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&mut self, message: &str) -> ParseError {
        let token = self.peek();
        if token.token_type == TokenType::EOF {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        }
        self.had_error = true;
        ParseError
    }

    // ---- grammar rules, so far: expression -> factor -> unary -> primary ----

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.logic_or()
    }

    // one left-associative level: operand ( operator operand )*
    fn binary_level(
        &mut self,
        operators: &[TokenType],
        next: fn(&mut Parser) -> Result<Expr, ParseError>,
    ) -> Result<Expr, ParseError> {
        let mut expr = next(self)?;

        while self.match_token(operators) {
            let operator = self.previous().clone();
            let right = next(self)?;
            // the tree built so far becomes the LEFT child: this is the left-associative fold
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(&[TokenType::OKAYA], Parser::logic_and)
    }

    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(&[TokenType::AT], Parser::equality)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(&[TokenType::NOTEQUAL, TokenType::EQUALEQUAL], Parser::comparison)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(
            &[TokenType::GREATER, TokenType::GREATEREQUAL, TokenType::LESS, TokenType::LESSEQUAL],
            Parser::term,
        )
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(&[TokenType::MINUS, TokenType::PLUS], Parser::factor)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        self.binary_level(&[TokenType::SLASH, TokenType::STAR, TokenType::MODULO], Parser::unary)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::NOT, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?; // recursive, so !!x and --5 work
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.primary()
    }

    // program → ( expression ";" )* EOF
    pub fn parse_program(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::new();
        while !self.is_at_end() {
            let expr = self.expression()?;
            self.consume(TokenType::SEMICOLON, "Expect ';' after expression.")?;
            exprs.push(expr);
        }
        Ok(exprs)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::NUMBER, TokenType::STRING]) {
            let value = self.previous().literal.clone().expect("literal token must carry a Value");
            return Ok(Expr::Literal(value));
        }

        if self.match_token(&[TokenType::TOTOO]) {
            return Ok(Expr::Literal(Value::Boolean(true)));
        }
        if self.match_token(&[TokenType::MALI]) {
            return Ok(Expr::Literal(Value::Boolean(false)));
        }
        if self.match_token(&[TokenType::WALA]) {
            return Ok(Expr::Literal(Value::Nil));
        }

        if self.match_token(&[TokenType::LPAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RPAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(self.error("Expect expression."))
    }
}