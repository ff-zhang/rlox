use std::convert::TryInto;

use crate::chunk::{Chunk, Opcode};

pub trait Debug {
    fn disassemble(&self, name: &str);
}

impl Debug for Chunk {
    fn disassemble(&self, name: &str) {
        println!("==== {name} ====");
        let mut offset = 0;
        while offset < self.code.len() {
            disassemble_instruction(self, &mut offset);
        }
    }
}

fn disassemble_instruction(this: &Chunk, offset: &mut usize) {
    print!("{offset:04} ");
    if *offset > 0 && this.lines[*offset - 1] == this.lines[*offset - 1] { print!("   | "); }
    else { print!("{:04} ", this.lines[*offset]) }

    let instruction = this.code[*offset].try_into().unwrap();
    match instruction {
        Opcode::OpConstant => constant_instruction("OP_CONSTANT", this, offset),
        Opcode::OpReturn => simple_instruction("OP_RETURN", offset),
    }
}

fn constant_instruction(name: &str, this: &Chunk, offset: &mut usize) {
    let constant = this.code.get(*offset + 1).unwrap();
    let value = this.constants[*constant as usize];
    println!("{name:<16} {constant} '{value}'");
    *offset += 2;
}

fn simple_instruction(name: &str, offset: &mut usize) {
    println!("{name}");
    *offset += 1;
}
