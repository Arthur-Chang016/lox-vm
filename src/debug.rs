use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: &Vec<OpCode>, name: &str) {
    println!("== {name} ==");
    
    let mut offset = 0;
    while offset < chunk.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Vec<OpCode>, offset: usize) ->usize {
    print!("{:04} ", offset);
    
    let instruction = chunk[offset];
    
    match instruction {
        OpCode::OpReturn => return simple_instruction("OP_RETURN", offset),
        
        _ => {
            println!("Unknown opcode {}", instruction as u8);
            return offset + 1;
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    return offset + 1;
}