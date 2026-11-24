use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[derive(PartialEq)]
enum TokenType {
    Return,
    IntLit,
    Semi,
}

struct Token {
    token_type: TokenType,
    token_value: Option<String>,
}

fn printToken(token: &Token) {
    if (token.token_type == TokenType::Return) {
        println!("Return");
    } else if (token.token_type == TokenType::IntLit) {
        println!("Integer Literal");
    } else {
        println!("Semicolon");
    }
}

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

fn tokenize(str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let chars: Vec<char> = str.chars().collect();
    let mut buf = String::new();
    let mut i: usize = 0;

    while i < chars.len() {
        let c: char = chars[i];
        //Check if it is keyword
        if (c.is_alphabetic() || c == '_') {
            buf.push(c);
            i += 1;
            while i < chars.len() && chars[i].is_alphanumeric() {
                buf.push(chars[i]);
                i += 1;
            }
            i -= 1;

            if buf == "conclude" {
                tokens.push(Token {
                    token_type: TokenType::Return,
                    token_value: None,
                });
                buf.clear();
            } else {
                println!("Some Error Occurred when checking Keyword!");
                std::process::exit(1);
            }
        }
        //Check if it is Integer Literal
        else if c.is_digit(10) {
            buf.push(c);
            i += 1;

            while i < chars.len() && chars[i].is_digit(10) {
                buf.push(chars[i]);
                i += 1;
            }

            i -= 1;

            tokens.push(Token {
                token_type: TokenType::IntLit,
                token_value: Some(std::mem::take(&mut buf)),
            });
        }
        //Check for Semi-Colon
        else if c == ';' {
            tokens.push(Token {
                token_type: TokenType::Semi,
                token_value: None,
            });
        }
        //Skip Spaces
        else if c.is_whitespace() {
        } else {
            println!("An Error Occurred during tokenization!");
        }

        i += 1;
    }

    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: beuro <filename.beuro>");
        return;
    }

    let filename = String::from(&args[1]);
    let contents = fs::read_to_string(filename).expect("Should have been able to read the File");

    let tokens: Vec<Token> = tokenize(&contents);
    let mut asm = tokens_to_asm(&tokens);
    let mut file = File::create("out.asm").unwrap();
    file.write_all(asm.as_bytes()).unwrap();

    let exec_command = Command::new("nasm")
        .args(["-f", "elf64", "out.asm", "-o", "out.o"])
        .status();

    let exec_command_2 = Command::new("ld").args(["-o", "out", "out.o"]).status();
}
