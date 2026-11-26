mod token;
mod tokenizer;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use token::Token;
use token::TokenType;
use tokenizer::Tokenizer;

fn tokens_to_asm(tokens: &Vec<Token>) -> String {
    let mut output: String = String::new();

    output.push_str("global _start\n_start:\n");

    let mut i: usize = 0;
    while i < tokens.len() {
        let token: &Token = &tokens[i];

        if token.token_type == TokenType::Return {
            if i + 1 < tokens.len() && tokens[i + 1].token_type == TokenType::IntLit {
                if i + 2 < tokens.len() && tokens[i + 2].token_type == TokenType::Semi {
                    let value = tokens[i + 1].token_value.as_ref().unwrap();

                    output.push_str(&format!(
                        "    mov rax, 60\n    mov rdi, {}\n    syscall\n",
                        value
                    ));
                }
            }
        }
        i += 1;
    }

    output
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: beuro <filename.beuro>");
        return;
    }

    let filename = String::from(&args[1]);
    let contents = fs::read_to_string(filename).expect("Should have been able to read the File");

    let mut tokenizer = Tokenizer::new(contents.chars().collect());

    let tokens: Vec<Token> = tokenizer.tokenize();
    let asm = tokens_to_asm(&tokens);
    let mut file = File::create("out.asm").unwrap();
    file.write_all(asm.as_bytes()).unwrap();

    let _exec_command = Command::new("nasm")
        .args(["-f", "elf64", "out.asm", "-o", "out.o"])
        .status();

    let _exec_command_2 = Command::new("ld").args(["-o", "out", "out.o"]).status();
}
