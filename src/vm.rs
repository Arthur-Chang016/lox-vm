use crate::chunk::Chunk;

pub struct VM {
    chunk: Chunk,
}

impl VM {
    
    pub fn new() -> VM {
        return VM {
            chunk: Chunk::new(),
        }
    }
    
    pub fn init_vm(&self) {
        
    }
    
    pub fn free_vm(&self) {
        
    }
    
}