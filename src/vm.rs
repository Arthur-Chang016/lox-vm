
use crate::chunk::Chunk;

pub struct VM {
    pub chunk: Box<Chunk>,
}

impl VM {
    
    pub fn new(c: Chunk) -> VM {
        return VM {
            chunk: Box::new(c),
        }
    }
    
    pub fn init_vm(&self) {
        
    }
    
    pub fn free_vm(&self) {
        
    }
    
}

// pub fn interpret(chunk: &Chunk) -> InterpreResult {
//     None
// }