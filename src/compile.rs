use std::str;

use crate::scanner::{
    Scanner, TokenType
};

pub fn compile(source: &[u8]) {
    let mut scanner = Scanner::new(source);

    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:02} '{}'", token.kind as u8, str::from_utf8(token.slice).unwrap());

        if token.kind == TokenType::Eof { break; }
    }
}
