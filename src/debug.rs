use crate::chunk::{OpCode, Chunk};

impl Chunk {
    
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    
    pub fn disassemble_instruction(&self, offset: usize) ->usize {
        print!("{:04} ", offset);
        
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match instruction {
            x if x == OpCode::OpConstant as u8 => return constant_instruction("OP_CONSTANT", &self, offset),
            x if x == OpCode::OpReturn as u8 => return simple_instruction("OP_RETURN", offset),
            
            _ => {
                println!("Unknown opcode {}", instruction as u8);
                return offset + 1;
            }
        }
    }
}


fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    return offset + 1;
}

pub(crate) fn print_value(value: f64) {
    print!("{:.2}", value);
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:04} '", name, constant);
    print_value(chunk.constants[constant as usize]);
    println!("'");
    return offset + 2;
}