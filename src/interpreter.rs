use std::io::Read;

use crate::{
    lexer::{Lexer, Token, TokenKind},
    memory::{Cell, PageTableMemory},
};

pub fn run(source: &str) {
    let mut memory = PageTableMemory::new(1);
    let mut pointer = 0;
    let tokens = Lexer::new(source).collect::<Vec<Token>>();
    let mut pos = 0;
    let mut read_buf = [0; 1];
    let mut stdin = std::io::stdin();
    while pos < tokens.len() {
        match tokens[pos].kind {
            TokenKind::Plus => {
                if memory[pointer] == Cell::MAX {
                    memory[pointer] = 0;
                } else {
                    memory[pointer] += 1;
                }
            }
            TokenKind::Minus => {
                if memory[pointer] == Cell::MIN {
                    memory[pointer] = Cell::MAX;
                } else {
                    memory[pointer] -= 1;
                }
            }
            TokenKind::Left => pointer -= 1,
            TokenKind::Right => pointer += 1,
            TokenKind::Open => {
                if memory[pointer] == 0 {
                    while tokens[pos].kind != TokenKind::Close {
                        pos += 1;
                    }
                }
            }
            TokenKind::Close => {
                if memory[pointer] != 0 {
                    while tokens[pos].kind != TokenKind::Open {
                        pos -= 1;
                    }
                }
            }
            TokenKind::Print => print!("{}", memory[pointer] as char),
            TokenKind::Read => {
                stdin
                    .read_exact(&mut read_buf)
                    .expect("Failed to read from stdin");
                memory[pointer] = read_buf[0];
            }
            TokenKind::Ignored => {}
        }
        pos += 1;
    }
}
