
use debug::disassemble_chunk;

use crate::chunk::OpCode;
// use crate::debug;

mod chunk;

mod debug;

fn main() {
    let mut chunk: Vec<OpCode> = Vec::new();
    
    // let tmp = mem::size_of::<OpCode>();
    
    // println!("{tmp}");
    
    chunk.push(OpCode::OpReturn);
    
    // println!("{chunk. }");
    
    
    disassemble_chunk(&chunk, "test chunk");
    
    // println!("Hello, world!");
}
