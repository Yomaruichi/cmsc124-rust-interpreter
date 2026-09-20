use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
}

impl Scanner {
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

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            self.had_error = true;
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_literal(TokenType::STRING, value);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_literal(TokenType::NUMBER, value);
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

    pub fn advance (&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        return ch;
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: None,
            line: self.line,
        })

    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: Some(literal),
            line: self.line,
        })
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = Scanner::keyword_lookup(&text).unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token_type);
    }

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
            '/' => self.add_token(TokenType::SLASH),
            ' ' | '\r' | '\t' => {},
            '\n' => {self.line = self.line + 1}
            '"' => self.string(),
            '0'..='9' => self.number(),
            c if c.is_alphanumeric() || c == '_' => self.identifier(),
            _ => self.error(input)
        }
    }

    fn keyword_lookup(text: &str) -> Option<TokenType> {
        match text {
            "func"  => Some(TokenType::FUNC),
            "const" => Some(TokenType::CONST),
            "let"   => Some(TokenType::LET),
            "if"    => Some(TokenType::IF),
            "else"  => Some(TokenType::ELSE),
            "while" => Some(TokenType::WHILE),
            "do"    => Some(TokenType::DO),
            "for"   => Some(TokenType::FOR),
            "and"   => Some(TokenType::AND),
            "or"    => Some(TokenType::OR),
            "break" => Some(TokenType::BREAK),
            "continue" => Some(TokenType::CONTINUE),
            "true"  => Some(TokenType::TRUE),
            "false" => Some(TokenType::FALSE),
            "none"  => Some(TokenType::NONE),
            "import"=> Some(TokenType::IMPORT),
            "print" => Some(TokenType::PRINT),
            "input" => Some(TokenType::INPUT),
            "switch"=> Some(TokenType::SWITCH),
            "case"  => Some(TokenType::CASE),
            "default"=> Some(TokenType::DEFAULT),
            "return"=> Some(TokenType::RETURN),
            _ => None
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn error(&mut self, ch: char) {
        eprintln!("[line {}] Error: Unexpected character '{}'", self.line, ch);
        self.had_error = true;
    }


}