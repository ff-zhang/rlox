use crate::chunk::{Chunk, Opcode};

pub trait Debug {
    fn disassemble(&self, name: &str);
}

impl Debug for Chunk {
    fn disassemble(&self, name: &str) {
        println!(" == {name} ==");
        let mut offset = 0;
        while offset < self.code.len() {
            disassemble_instruction(self, &mut offset);
        }
    }
}

fn disassemble_instruction(this: &Chunk, offset: &mut usize) {
    print!("{offset:04} ");

    let instruction = this.code.get(*offset).unwrap();
    match instruction {
        Opcode::OpReturn => simple_instruction("OP_RETURN", offset),
    }
}

fn simple_instruction(name: &str, offset: &mut usize) {
    println!("{name}");
    *offset += 1;
}