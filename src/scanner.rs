
#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar,
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    // Keywords.
    TokenAnd, TokenClass, TokenElse, TokenFalse,
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,
    // Others
    TokenError, TokenEmpty, TokenEof
}

pub struct Scanner<'a> {
    pub start: usize,
    pub current: usize,
    pub line: i32,
    pub source: &'a str,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub type_: TokenType,
    // pub start: usize,
    // pub length: i32,
    pub content: &'a str,
    pub line: i32,
}

impl Scanner<'_> {
    pub fn new(s: &str) -> Scanner {
        return Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: s,
        }
    }
    
    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    
    fn make_content(&self) -> &str {
        return &self.source[self.start .. self.current];
    }
    
    pub fn advance(&mut self) -> char {
        let ret = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        return ret;
    }
    
    pub fn peek(&self) -> char {
        if self.current == self.source.as_bytes().len() {  // simulate C for null termination
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }
    
    pub fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    
    pub fn peek_next(&mut self) -> char {
        return if self.is_at_end() {
            '\0'
        } else {
            self.source.as_bytes()[self.current + 1] as char
        }
    }
    
    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            if c == ' ' || c == '\r' || c == '\t' {
                self.advance();
            } else if c == '\n' {
                self.line += 1;
                self.advance();
            } else if c == '/' {
                if self.peek_next() == '/' {  // comment
                    while self.peek() != '\n' && !self.is_at_end() {  // peek until next line
                        self.advance();
                    }
                } else {
                    return
                }
            } else {
                return
            }
        }
    }
    
    pub fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Token::error_token("Unterminated string.", self.line);
        }
        // The closing quote
        self.advance();
        return Token::make_token(TokenType::TokenString, self.line, self.make_content());
    }
    
    pub fn number(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }
        // look for fractional part
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();  // consume "."
            while is_digit(self.peek()) { self.advance(); }
        }
        return Token::make_token(TokenType::TokenNumber, self.line, self.make_content());
    }
    
    pub fn check_keyword(&self, start: usize, length: usize, rest: &str, type_: TokenType) -> TokenType {
        if self.current - self.start == start + length {
            let comp_str = &self.source[self.start + start .. self.start + start + length];
            if rest == comp_str {
                return type_;
            }
        }
        return TokenType::TokenIdentifier;
    }
    
    pub fn identifier_type(&self) -> TokenType {
        match self.source.as_bytes()[self.start] as char {
            'a' => return self.check_keyword(1, 2, "nd", TokenType::TokenAnd),
            'c' => return self.check_keyword(1, 4, "lass", TokenType::TokenClass),
            'e' => return self.check_keyword(1, 3, "lse", TokenType::TokenElse),
            'f' => if self.current - self.start > 1 {
                match self.source.as_bytes()[self.start + 1] as char {
                    'a' => return self.check_keyword(2, 3, "lse", TokenType::TokenFalse),
                    'o' => return self.check_keyword(2, 1, "r", TokenType::TokenFor),
                    'u' => return self.check_keyword(2, 1, "n", TokenType::TokenFun),
                    _ => {},
                }
            }
            'i' => return self.check_keyword(1, 1, "f", TokenType::TokenIf),
            'n' => return self.check_keyword(1, 2, "il", TokenType::TokenNil),
            'o' => return self.check_keyword(1, 1, "r", TokenType::TokenOr),
            'p' => return self.check_keyword(1, 4, "rint", TokenType::TokenPrint),
            'r' => return self.check_keyword(1, 5, "eturn", TokenType::TokenReturn),
            's' => return self.check_keyword(1, 4, "uper", TokenType::TokenSuper),
            't' => if self.current - self.start > 1 {
                match self.source.as_bytes()[self.start + 1] as char {
                    'h' => return self.check_keyword(2, 2, "is", TokenType::TokenThis),
                    'r' => return self.check_keyword(2, 2, "ue", TokenType::TokenTrue),
                    _ => {},
                }
            }
            'v' => return self.check_keyword(1, 2, "ar", TokenType::TokenVar),
            'w' => return self.check_keyword(1, 4, "hile", TokenType::TokenWhile),
            _ => {},
        }
        
        return TokenType::TokenIdentifier;
    }
    
    pub fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        return Token::make_token(self.identifier_type(), self.line, self.make_content());
    }
    
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return Token::make_token(TokenType::TokenEof, self.line, self.make_content());
        }
        
        let c = self.advance();
        if is_alpha(c) { return self.identifier(); }
        if is_digit(c) { return self.number(); }
            
        match c {
            '(' => return Token::make_token(TokenType::TokenLeftParen, self.line, self.make_content()),
            ')' => return Token::make_token(TokenType::TokenRightParen, self.line, self.make_content()),
            '{' => return Token::make_token(TokenType::TokenLeftBrace, self.line, self.make_content()),
            '}' => return Token::make_token(TokenType::TokenRightBrace, self.line, self.make_content()),
            ';' => return Token::make_token(TokenType::TokenSemicolon, self.line, self.make_content()),
            ',' => return Token::make_token(TokenType::TokenComma, self.line, self.make_content()),
            '.' => return Token::make_token(TokenType::TokenDot, self.line, self.make_content()),
            '-' => return Token::make_token(TokenType::TokenMinus, self.line, self.make_content()),
            '+' => return Token::make_token(TokenType::TokenPlus, self.line, self.make_content()),
            '/' => return Token::make_token(TokenType::TokenSlash, self.line, self.make_content()),
            '*' => return Token::make_token(TokenType::TokenStar, self.line, self.make_content()),
            
            '!' => return Token::make_token(if self.match_('=') { TokenType::TokenBangEqual } else { TokenType::TokenBang }, self.line, self.make_content()),
            '=' => return Token::make_token(if self.match_('=') { TokenType::TokenEqualEqual } else { TokenType::TokenEqual }, self.line, self.make_content()),
            '<' => return Token::make_token(if self.match_('=') { TokenType::TokenLessEqual } else { TokenType::TokenLess }, self.line, self.make_content()),
            '>' => return Token::make_token(if self.match_('=') { TokenType::TokenGreaterEqual } else { TokenType::TokenGreater }, self.line, self.make_content()),
            
            '"' => return self.string(),
            
            _ => {}
        }
        
        return Token::error_token("Unexpected character.", self.line);
    }
}

impl Token<'_> {
    pub fn error_token(message: &str, l: i32) -> Token {
        return Token {
            type_: TokenType::TokenError,
            content: message,
            line: l,
        }
    }
    
    pub fn make_token(t: TokenType, line: i32, content: &str) -> Token {
        return Token {
            type_: t,
            content: content,
            line: line,
        }
    }
    
    pub fn empty_token() -> Token<'static> {
        return Self::make_token(TokenType::TokenEmpty, 0, "");
    }
}

pub fn is_digit(c: char) -> bool {
    return '0' <= c && c <= '9';
}

pub fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}
