mod codegen;
mod nodes;
mod parser;
mod token;
mod tokenizer;

use codegen::CodeGenerator;
use nodes::NodeExit;
use parser::Parser;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use token::Token;
use tokenizer::Tokenizer;

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

    let mut parser: Parser = Parser::new(tokens);
    let tree: Option<NodeExit> = parser.parse();

    if tree.is_none() {
        println!("No Return Statement!");
        process::exit(1);
    }

    let generator: CodeGenerator = CodeGenerator::new(tree.unwrap());
    let asm: String = generator.generate_asm();

    let mut file = File::create("out.asm").unwrap();
    file.write_all(asm.as_bytes()).unwrap();

    let _exec_command = Command::new("nasm")
        .args(["-f", "elf64", "out.asm", "-o", "out.o"])
        .status();

    let _exec_command_2 = Command::new("ld").args(["-o", "out", "out.o"]).status();
}
