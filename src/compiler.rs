use crate::{scanner::{Scanner, TokenType, Token}, chunk::{Chunk, OpCode}, vm::VM};

pub struct Parser<'a> {
    pub current: Token<'a>,
    pub previous: Token<'a>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser<'_> {
    
    pub fn new() -> Parser<'static> {
        return Parser {
            current: Token::empty_token(),
            previous: Token::empty_token(),
            had_error: false,
            panic_mode: false,
        }
    }
    
    pub fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current, message);
        self.had_error = true;
    }
    
    pub fn error(&mut self,  message: &str) {
        self.error_at(&self.previous, message);
        self.had_error = true;
    }
    
    pub fn error_at(&self, token: &Token, message: &str) {
        eprint!("[line {}] Error", token.line);
        if token.type_ == TokenType::TokenEof {
            eprint!(" at end");
        } else if token.type_ == TokenType::TokenError {
            // Nothing
        } else {
            eprint!(" at '{}'", token.content);
        }
        eprintln!(": {message}");
        if self.panic_mode {
            return
        }
    }
    
    pub fn number(&mut self, chunk: &mut Chunk) {
        let value = self.previous.content.parse::<f64>().expect("Not a number");
        chunk.emit_constant(value, self);
    }
    
}


pub fn advance<'a, 'b>(parser: &'a mut Parser<'b>, scanner: &'a mut Scanner<'b>) {
    parser.previous = parser.current;
    
    let scanner_ptr = scanner as *mut Scanner;
    let parser_ptr = parser as *mut Parser;
    
    loop {
        unsafe {
            let token = (*scanner_ptr).scan_token();
            (*parser_ptr).current = token;
        }
        
        if parser.current.type_ != TokenType::TokenError {
            break;
        }
        parser.error_at_current(parser.current.content);
    }
}

pub fn consume<'a, 'b>(parser: &'a mut Parser<'b>, type_: TokenType, message: &str, scanner: &'a mut Scanner<'b>) {
    if parser.current.type_ == type_ {
        advance(parser, scanner);
        return
    }
    parser.error_at_current(message);
}

pub fn compile(source: &str, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();
    advance(&mut parser, &mut scanner);
    expression();
    consume(&mut parser, TokenType::TokenEof, "Expect end of expression.", &mut scanner);
    end_compiler(chunk, &parser);
    return !parser.had_error;
}

pub fn grouping<'a, 'b>(parser: &'a mut Parser<'b>, scanner: &'a mut Scanner<'b>) {
    expression();
    consume(parser, TokenType::TokenRightParen, "Expect ')' after expression.", scanner);
}

pub fn unary<'a, 'b>(parser: &'a mut Parser<'b>, scanner: &'a mut Scanner<'b>, chunk: &mut Chunk) {
    let operator_type = parser.previous.type_;
    // Compile this operand
    expression();
    
    // Emit the operator instruction
    match operator_type {
        x if x == TokenType::TokenMinus => chunk.emit_byte(OpCode::OpNegate as u8, parser),
        _ => { return }  // Unreachable
    }
}

pub fn expression() {
    
}

pub fn end_compiler(chunk: &mut Chunk, parser: &Parser) {
    chunk.emit_return(parser);
}

impl Chunk {
    pub fn emit_byte(&mut self, byte: u8, parser: &Parser) {
        self.write_chunk(byte, parser.previous.line);
    }
    
    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8, parser: &Parser) {
        self.emit_byte(byte1, parser);
        self.emit_byte(byte2, parser);
    }
    
    pub fn emit_constant(&mut self, value: f64, parser: &mut Parser) {
        let constant = self.make_constant(value, parser);
        self.emit_bytes(OpCode::OpConstant as u8, constant, parser);
    }
    
    pub fn emit_return(&mut self, parser: &Parser) {
        self.emit_byte(OpCode::OpReturn as u8, parser);
    }
    
    pub fn make_constant(&mut self, value: f64, parser: &mut Parser) -> u8 {
        let constant = self.add_constant(value);
        if constant > u8::MAX as usize {
            parser.error("Too many constants in one chunk.");
            return 0;
        }
        return constant as u8;
    }
}


