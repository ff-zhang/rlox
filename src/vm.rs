use std::{
    error::Error,
    fmt::Formatter,
};

use crate::{
    chunk::{Chunk, Opcode, Value},
    compile::compile,
    debug::disassemble_instruction,
};

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct CompileError;

impl Error for CompileError {}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompileError")
    }
}

#[derive(Debug)]
pub struct RuntimeError;

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuntimeError")
    }
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError(CompileError),
    RuntimeError(RuntimeError),
}

impl Error for InterpretError {}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::CompileError(err) => write!(f, "{}", err),
            InterpretError::RuntimeError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    chunk: Option<&'a Chunk>,
    ip: *const u8,
    stack: [Value; STACK_MAX],
    top: *mut Value,
}

impl<'a> VirtualMachine<'a> {
    pub fn init(&mut self) {
        self.reset_stack();
    }

    pub fn interpret(&mut self, source: &[u8]) -> Result<(), InterpretError> {
        compile(source);
        Ok(())
    }

    pub fn push(&mut self, value: Value) {
        // TODO: use vec::push()
        unsafe {
            self.top.write(value);
            self.top = self.top.offset(1);
        }
    }

    pub fn pop(&mut self) -> Value {
        // TODO: use vec::pop()
        unsafe {
            self.top = self.top.offset(-1);
            self.top.read()
        }
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        macro_rules! binary_op {
            ($op:tt) => {{
                let b = self.pop();
                let a = self.pop();
                self.push(a $op b);
            }};
        }

        loop {
            #[cfg(debug_assertions)]
            {
                print!("            ");
                let mut slot = self.stack.as_ptr();
                while slot < self.top {
                    print!("[ {} ]", unsafe { slot.read() });
                    unsafe { slot = slot.offset(1) };
                }
                println!();

                let mut offset = self.ip as usize - self.chunk.unwrap().code.as_ptr() as usize;
                disassemble_instruction(self.chunk.unwrap(), &mut offset);
            }

            let instruction = self.read_byte().try_into().unwrap();
            match instruction {
                Opcode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                },
                Opcode::Add => binary_op!(+),
                Opcode::Subtract => binary_op!(-),
                Opcode::Multiply => binary_op!(*),
                Opcode::Divide => binary_op!(/),
                Opcode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                },
                Opcode::Return => {
                    println!("{}", self.pop());
                    return Ok(());
                },
            }
        }
    }

    fn reset_stack(&mut self) {
        self.top = self.stack.as_mut_ptr();
    }

    // All below are macros originally
    fn read_byte(&mut self) -> u8 {
        unsafe {
            let instruction = self.ip.read();
            self.ip = self.ip.offset(1);
            instruction
        }
    }

    fn read_constant(&mut self) -> Value {
        let constant = self.read_byte() as usize;
        *self.chunk.unwrap().constants.get(constant).unwrap()
    }
}
