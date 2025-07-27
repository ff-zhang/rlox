#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    OpReturn,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Opcode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: Opcode) {
        self.code.push(byte)
    }
    
    pub fn free(&mut self) {
        self.code.clear()
    }
}
