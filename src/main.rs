
use chunk::Chunk;
use debug::disassemble_chunk;
use vm::VM;

use crate::chunk::OpCode;
// use crate::debug;

mod chunk;
mod debug;
mod vm;


fn main() {
    let vm = VM::new();
    vm.init_vm();
    
    
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant as u8, 123);
    chunk.write_chunk(constant as u8, 123);
    
    chunk.write_chunk(OpCode::OpReturn as u8, 123);
    
    
    disassemble_chunk(&chunk, "test chunk");
    
    vm.free_vm();
}
