mod chunk;
mod debug;

use chunk::{Chunk, Opcode};
use debug::Debug;

fn main() {
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);
    chunk.write(Opcode::OpConstant as u8, 123);
    chunk.write(constant, 123);
    chunk.write(Opcode::OpReturn as u8, 123);
    chunk.disassemble("Test Chunk");
    
    chunk.free();
}
