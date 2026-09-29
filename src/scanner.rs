use crate::token::{Token, TokenType, Value};

// struct maintaining character source, current offset, line tracking, and accumulated tokens
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
}

impl Scanner {
    // initializes new Scanner with initial values 
    pub fn new_string(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    // String token builder, consumes characters up to ending '"', emits TokenType::STRING or returns 65 for unterminated string
    fn string(&mut self) {
        let mut value = String::new();

        // advance through characters
        while self.peek() != '"' && !self.is_at_end() && self.peek() != '\n' {
            // checks for special escape command
            if self.peek() == '\\' {
                self.advance();

                if self.is_at_end() || self.peek() == '\n' {
                    break;
                }

                let escaped = self.advance();
                match escaped {
                    'n' => value.push('\n'),
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    other => {
                        self.report(
                            self.line,
                            &format!("Invalid escape sequence '\\{}'. Supported: \\n, \\\", \\\\.", other),
                        );
                        value.push(other);
                    }
                }
                // pushes current character into value
            } else {
                value.push(self.advance());
            }
        }

        // returns error 65
        if self.peek() != '"' {
            self.report(self.line, "Unterminated string. Strings must end on the same line.");
            return;
        }

        self.advance();
        self.add_token_with_literal(TokenType::STRING, Value::String(value));
    }

    // number token builder
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        if self.peek() == '.' && !self.peek_next().is_ascii_digit() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = text.parse().expect("scanned digits always form a valid f64");
        self.add_token_with_literal(TokenType::NUMBER, Value::Number(value));
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token { 
            token_type: TokenType::EOF, 
            lexeme: String::new(), 
            literal: None, 
            line: self.line });

        &self.tokens
    }

    // helper, makes current char the target and advances looker
    pub fn advance (&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        return ch;
    }

    // helper, advances and returns true if char matches, used for comments
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    // helper, 
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    // helper, 
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    
    // helper, push to tokens vector for single, dual, and keyword tokens
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: None,
            line: self.line,
        })

    }

    // helper, push to tokens vector for tokens with literals ( Identifier, Numbers, Boolean, Nil )
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Value) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: Some(literal),
            line: self.line,
        })
    }

    // identifier token builder
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = Scanner::keyword_lookup(&text).unwrap_or(TokenType::IDENTIFIER);

        match token_type {
            TokenType::TOTOO => self.add_token_with_literal(token_type, Value::Boolean(true)),
            TokenType::MALI => self.add_token_with_literal(token_type, Value::Boolean(false)),
            TokenType::WALA => self.add_token_with_literal(token_type, Value::Nil),
            _ => self.add_token(token_type),
        }
    }

    // runs first, takes first char and match from given possible available token types
    pub fn scan_token(&mut self) {
        let input = self.advance();

        match input {
            '(' => self.add_token(TokenType::LPAREN),
            ')' => self.add_token(TokenType::RPAREN),
            '{' => self.add_token(TokenType::LCURLY),
            '}' => self.add_token(TokenType::RCURLY),
            ',' => self.add_token(TokenType::COMMA),
            ';' => self.add_token(TokenType::SEMICOLON),
            '=' => {
                let t = if self.match_char('=') { TokenType::EQUALEQUAL } else { TokenType::EQUAL };
                self.add_token(t);
            }
            '!' => {
                let t = if self.match_char('=') { TokenType::NOTEQUAL } else { TokenType::NOT };
                self.add_token(t);
            }
            '<' => {
                let t = if self.match_char('=') { TokenType::LESSEQUAL } else { TokenType::LESS };
                self.add_token(t);
            }
            '>' => {
                let t = if self.match_char('=') { TokenType::GREATEREQUAL } else { TokenType::GREATER };
                self.add_token(t);
            }
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            '*' => self.add_token(TokenType::STAR),
            '/' => {
                        if self.match_char('/') {
                            self.line_comment();
                        } else if self.match_char('*') {
                            self.block_comment();
                        } else {
                            self.add_token(TokenType::SLASH);
                        }
                    },
            '%' => self.add_token(TokenType::MODULO),
            ' ' | '\r' | '\t' => {},
            '\n' => {self.line = self.line + 1}
            '"' => self.string(),
            '.' => {
                    if self.peek().is_ascii_digit() {
                        self.leading_decimal_number();
                    } else {
                        self.unexpected_char(input);
                    }
                }
            '0'..='9' => self.number(),
            c if c.is_alphanumeric() || c == '_' => self.identifier(),
            _ => self.unexpected_char(input)
        }
    }

    // helper, checks for keyword tokens
    fn keyword_lookup(text: &str) -> Option<TokenType> {
        match text {
            "gawa"  => Some(TokenType::GAWA),
            "uri"   => Some(TokenType::URI),
            "tiyak" => Some(TokenType::TIYAK),
            "itakda"   => Some(TokenType::ITAKDA),
            "kung"    => Some(TokenType::KUNG),
            "kundi"  => Some(TokenType::KUNDI),
            "habang" => Some(TokenType::HABANG),
            "gawin"    => Some(TokenType::GAWIN),
            "tuwing"   => Some(TokenType::TUWING),
            "at"   => Some(TokenType::AT),
            "okaya"    => Some(TokenType::OKAYA),
            "tigil" => Some(TokenType::TIGIL),
            "ituloy" => Some(TokenType::ITULOY),
            "totoo"  => Some(TokenType::TOTOO),
            "mali" => Some(TokenType::MALI),
            "wala"  => Some(TokenType::WALA),
            "isama"=> Some(TokenType::ISAMA),
            "ipakita" => Some(TokenType::IPAKITA),
            "ipasok" => Some(TokenType::IPASOK),
            "piliin"=> Some(TokenType::PILIIN),
            "kapag"  => Some(TokenType::KAPAG),
            "edi"=> Some(TokenType::EDI),
            "ibalik"=> Some(TokenType::IBALIK),
            _ => None
        }
    }

    // helper, checks length of file against current index
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // helper, formats error return and reports if an actual error happened
    fn report(&mut self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
        self.had_error = true;
    }

    // helper, returns error hints
    fn unexpected_char(&mut self, ch: char) {
        let message = match ch {
            '&' => "Unexpected character '&'. Use 'at' for logical and.".to_string(),
            '|' => "Unexpected character '|'. Use 'okaya' for logical or.".to_string(),
            '.' if self.peek().is_ascii_digit() => "Unexpected character '.'. A decimal needs a digit before the point: write 0.5, not .5.".to_string(),
            _ => format!("Unexpected character '{}'.", ch),
        };
        self.report(self.line, &message);
    }

    // one line comments
    fn line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    // 
    fn block_comment(&mut self) {
    let start_line = self.line;
    let mut depth = 1;

    while depth > 0 {
        if self.is_at_end() {
            self.report(start_line, "Unterminated block comment.");
            return;
        }

        if self.peek() == '\n' {
            self.line += 1;
        }

        if self.peek() == '/' && self.peek_next() == '*' {
            self.advance(); // consume '/'
            self.advance(); // consume '*'
            depth += 1;
            continue;
        }

        if self.peek() == '*' && self.peek_next() == '/' {
            self.advance(); // consume '*'
            self.advance(); // consume '/'
            depth -= 1;
            continue;
        }

        self.advance();
    }
}

// Helper to scan numbers that start with a leading dot like .5
fn leading_decimal_number(&mut self) {
    // Consume all digits following the '.'
    while self.peek().is_ascii_digit() {
        self.advance();
    }

    let text: String = self.source[self.start..self.current].iter().collect();
    let value: f64 = text.parse().unwrap_or(0.0);
    self.add_token_with_literal(TokenType::NUMBER, Value::Number(value));
}

}