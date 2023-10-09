
use crate::chunk::{Chunk, OpCode};

use crate::debug::print_value;

pub enum InterpreResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

const STACK_MAX: usize = 256;

pub struct VM {
    pub chunk: Box<Chunk>,
    pub ip: usize,  // program counter: always point to the next instruction
    pub stack: Vec<f64>,
    pub stack_top: usize,  // stack pointer
}

impl VM {
    
    pub fn new(c: Chunk) -> VM {
        return VM {
            chunk: Box::new(c),
            ip: 0,
            stack: vec![0.0; STACK_MAX],
            stack_top: 0,
        }
    }
    
    pub fn init_vm(&self) {
        
    }
    
    pub fn free_vm(&self) {
        
    }
    
    pub fn push(&mut self, value: f64) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }
    
    pub fn pop(&mut self) -> f64 {
        self.stack_top -= 1;
        return self.stack[self.stack_top];
    }
    
    pub fn interpret(&mut self) -> InterpreResult {
        return self.run();
    }
    
    fn read_byte(&mut self) -> u8 {
        let ret = self.chunk.code[self.ip];
        self.ip += 1;
        return ret;
    }
    
    fn read_constant(&mut self) -> f64 {
        let index = self.read_byte() as usize;
        return self.chunk.constants[index];
    }
    
    pub fn run(&mut self) -> InterpreResult {
        loop {
            // debug
            // self.chunk.disassemble_instruction(self.ip);
            
            let instruction = self.read_byte();
            match instruction {
                x if x == OpCode::OpConstant as u8 => {
                    let constant = self.read_constant();
                    print_value(constant);
                    println!();
                }
                x if x == OpCode::OpReturn as u8 => return InterpreResult::InterpretOk,
                
                _ => {},
            }
        }
        
    }
    
}

