mod chunk;
mod debug;

use chunk::{Chunk, Opcode};
use debug::Debug;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Opcode::OpReturn);
    chunk.disassemble("Test Chunk");
    chunk.free();
}
