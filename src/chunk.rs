use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Constant),
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Subtract),
            3 => Ok(Opcode::Multiply),
            4 => Ok(Opcode::Divide),
            5 => Ok(Opcode::Negate),
            6 => Ok(Opcode::Return),
            _ => Err(format!("Unknown value {}", value)),
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
}
