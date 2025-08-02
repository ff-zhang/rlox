mod chunk;
mod debug;
mod vm;

use chunk::{Chunk, Opcode};
use debug::Debug;

use crate::vm::VirtualMachine;

fn main() {
    let mut vm: VirtualMachine = unsafe { std::mem::zeroed() };
    vm.init();

    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);
    
    chunk.write(Opcode::Add as u8, 123);
    
    let constant = chunk.add_constant(5.4);
    chunk.write(Opcode::Constant as u8, 123);
    chunk.write(constant, 123);
    
    chunk.write(Opcode::Divide as u8, 123);
    chunk.write(Opcode::Negate as u8, 123);
    
    chunk.write(Opcode::Return as u8, 123);

    #[cfg(debug_assertions)]
    chunk.disassemble("Test Chunk");
    
    vm.interpret(&chunk).unwrap();
    
    chunk.free();
}
