

use chunk::Chunk;
use vm::{VM, InterpreResult};
use std::{env, process, io::{stdin, self, stdout, Write}, fs::{self, read_to_string}};

use crate::{chunk::OpCode, vm::interpret};
// use crate::debug;

mod chunk;
mod debug;
mod vm;
mod compiler;

fn test() {
    let mut vm = VM::new(Chunk::new());
    
    // init const
    let constant = vm.chunk.add_constant(1.2);
    
    {  // add const and negate
        vm.chunk.write_chunk(OpCode::OpConstant as u8, 123);
        vm.chunk.write_chunk(constant as u8, 123);
        vm.chunk.write_chunk(OpCode::OpNegate as u8, 123);
    }
    {  // bin ops
        vm.chunk.write_chunk(OpCode::OpConstant as u8, 123);
        vm.chunk.write_chunk(constant as u8, 123);
        vm.chunk.write_chunk(OpCode::OpAdd as u8, 123);
        vm.chunk.write_chunk(OpCode::OpConstant as u8, 123);
        vm.chunk.write_chunk(constant as u8, 123);
        vm.chunk.write_chunk(OpCode::OpSubtract as u8, 123);
        vm.chunk.write_chunk(OpCode::OpConstant as u8, 123);
        vm.chunk.write_chunk(constant as u8, 123);
        vm.chunk.write_chunk(OpCode::OpMultiply as u8, 123);
        vm.chunk.write_chunk(OpCode::OpConstant as u8, 123);
        vm.chunk.write_chunk(constant as u8, 123);
        vm.chunk.write_chunk(OpCode::OpDivide as u8, 123);
    }
    {  // return
        vm.chunk.write_chunk(OpCode::OpReturn as u8, 123);
    }
    
    vm.run();
    
    vm.chunk.disassemble_chunk("test chunk");
    
    vm.free_vm();
}

fn repl() {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        
        match stdin().read_line(&mut line) {
            Ok(n) => {
                interpret(&line);
            }
            Err(error) => break,
        }
    }
    
}

fn run_file(path: &str) {
    match read_to_string(path) {
        Ok(source) => {
            let result = interpret(&source);
            if result == InterpreResult::InterpretCompileError {
                process::exit(65);
            } else if result == InterpreResult::InterpretRuntimeError {
                process::exit(70);
            }
        }
        Err(error) => {  // read file error
            eprintln!("{error}");
            eprintln!("Could not open file \"{path}\".\n");
            process::exit(74);
        }
    }
}

fn main() {
    // test();
    
    let mut vm = VM::new(Chunk::new());
    
    let args: Vec<String> = env::args().collect();
    
    // println!("{}", args.len());
    // for s in args {
    //     println!("{s}")
    // }
    
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {  // read file
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
    
    
}
