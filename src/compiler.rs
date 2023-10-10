
use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4}", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:2} '{}'\n", token.type_ as i32, token.content);
        
        if token.type_ == TokenType::TokenEof {
            break;
        }
    }
}