use std::mem;

use crate::chunk::OpCode;

pub mod chunk;

fn main() {
    let mut chunk: Vec<OpCode> = Vec::new();
    
    // let tmp = mem::size_of::<OpCode>();
    
    // println!("{tmp}");
    
    chunk.push(OpCode::OpReturn);
    
    // println!("{chunk. }");
    
    println!("Hello, world!");
}
