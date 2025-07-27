use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    OpConstant = 0,
    OpReturn = 1,
}

impl TryFrom<u8> for Opcode {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value { 
            x if x == Opcode::OpConstant as u8 => Ok(Opcode::OpConstant),
            x if x == Opcode::OpReturn as u8 => Ok(Opcode::OpReturn),
            _ => Err(()),
        }
    }
}

pub type Value = f64;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
    
    pub fn free(&mut self) {
        self.code.clear();
        self.constants.clear();
        self.lines.clear();
    }
}
