mod chunk;
mod debug;
mod vm;
mod compile;
mod scanner;

use std::{
    env,
    fs,
    io::{self, Write},
    process
};

use crate::vm::{CompileError, InterpretError, RuntimeError, VirtualMachine};

fn repl(vm: &mut VirtualMachine) {
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut line).is_err() || !line.is_ascii() {
            println!();
            break;
        }

        let _ = vm.interpret(line.as_bytes());
        line.clear();
    }
}

fn run_file(vm: &mut VirtualMachine, file_name: &str) {
    let source = match fs::read_to_string(&file_name) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("Could not read file \"{}\"", &file_name);
            process::exit(74);
        },
    };

    if !source.is_ascii() {
        eprintln!("Only the ASCII standard is supported");
        process::exit(74);
    }

    match vm.interpret(source.as_bytes()) {
        Ok(_) => {},
        Err(e) => match e {
            InterpretError::CompileError(CompileError) => process::exit(65),
            InterpretError::RuntimeError(RuntimeError) => process::exit(70),
        },
    };
}

fn main() {
    let mut vm: VirtualMachine = unsafe { std::mem::zeroed() };
    vm.init();

    let mut args = env::args();
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &*args.nth(1).unwrap()),
        _ => {
            eprintln!("Usage: {} [path]", args.nth(0).unwrap());
            process::exit(64);
        },
    }
}
