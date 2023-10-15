
use crate::chunk::{Chunk, OpCode};
use crate::debug::print_value;
use crate::compiler::compile;

#[derive(PartialEq)]
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
    
    pub fn debug(&self) {
        print!("          ");
        for i in 0 .. self.stack_top {
            print!("[ ");
            print_value(self.stack[i]);
            print!(" ]");
        }
        println!();
        self.chunk.disassemble_instruction(self.ip);
    }
    
    pub fn push(&mut self, value: f64) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }
    
    pub fn pop(&mut self) -> f64 {
        self.stack_top -= 1;
        return self.stack[self.stack_top];
    }
    
    pub fn binary_op<F>(&mut self, op: F) where F: Fn(f64, f64) -> f64 {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
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
            // TODO config debug mode macro
            // self.debug();
            
            let instruction = self.read_byte();
            match instruction {
                
                x if x == OpCode::OpConstant as u8 => {
                    let constant = self.read_constant();
                    // print_value(constant);
                    self.push(constant);
                    println!();
                }
                
                x if x == OpCode::OpAdd as u8 => {
                    self.binary_op(|a, b| a + b);
                }
                
                x if x == OpCode::OpSubtract as u8 => {
                    self.binary_op(|a, b| a - b);
                }
                
                x if x == OpCode::OpMultiply as u8 => {
                    self.binary_op(|a, b| a * b);
                }
                
                x if x == OpCode::OpDivide as u8 => {
                    self.binary_op(|a, b| a / b);
                }
                
                
                x if x == OpCode::OpNegate as u8 => {
                    let neg = -self.pop();
                    self.push(neg);
                }
                
                x if x == OpCode::OpReturn as u8 => {
                    print_value(self.pop());
                    println!();
                    return InterpreResult::InterpretOk;
                } 
                
                _ => {},
            }
        }
        
    }
}

pub fn interpret(source: &str) -> InterpreResult {
    let mut vm = VM::new(Chunk::new());
    
    if compile(source, &mut vm.chunk) == false {
        return InterpreResult::InterpretCompileError;
    }
    return vm.run();
}
