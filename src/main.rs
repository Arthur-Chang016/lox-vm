

use chunk::Chunk;
use vm::VM;

use crate::chunk::OpCode;
// use crate::debug;

mod chunk;
mod debug;
mod vm;


fn main() {
    
    
    let chunk = Chunk::new();
    // let chunk_ref = chunk.borrow_mut();
    
    let mut vm = VM::new(chunk);
    vm.init_vm();
    
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
    
    
    vm.chunk.disassemble_chunk("test chunk");
    
    vm.free_vm();
}
