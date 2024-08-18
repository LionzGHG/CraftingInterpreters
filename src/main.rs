use std::{fs, io::{self, stdin, BufRead, BufReader}};

use lexer::{tokens::Token, Lexer};
use parser::{ast::Expr, ast_printer::AstPrinter, Parser};

pub mod lexer;
pub mod parser;
pub mod interpreter;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() > 1 {
        println!("Usage: rogue [script]");
        std::process::exit(64)
    } else if args.len() == 1 {
        run_file(&args[0]).expect("Failed to run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    if !path.contains(".rogue") {
        let new_path: String = String::from(format!("{path}.rogue").as_str());
        let bytes: Vec<u8> = fs::read(&new_path)?;
        run(String::from_utf8(bytes).unwrap());
    } 
    else {
        let bytes: Vec<u8> = fs::read(path)?;
        run(String::from_utf8(bytes).unwrap());
    }
    Ok(())
}

fn run_prompt() {
    let input: std::io::Stdin = stdin();
    let mut reader: BufReader<std::io::Stdin> = BufReader::new(input);

    loop {
        print!("> ");
        
        let mut buffer: String = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {},
            Err(_) => break,
        }

        run(buffer);
    }
}

fn run(source: String) {
    let mut lexer: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.tokenize();

    for token in &tokens {
        println!("{token:?}");
    }

    let mut parser: Parser = Parser::new(tokens);
    let expr: Box<dyn Expr> = parser.parse();

    println!("expr = {}", AstPrinter().print(&*expr));
}

pub fn report(line: usize, where_: &str, msg: &str) -> ! {
    println!("[line {line}] Error {where_}: {msg}");
    std::process::exit(65)
}