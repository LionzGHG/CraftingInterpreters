use core::str;
use std::{fs, io::{self, BufRead, Write}, path::Path};

use scanner::{tokens::Token, Scanner};

pub mod scanner;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() > 1 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(args[0].clone()).expect("Failed to run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: String) -> io::Result<()> {
    let bytes: Vec<u8> = fs::read(Path::new(&path))?;
    let contents: &str = str::from_utf8(&bytes).expect("Failed to convert bytes to string.");
    run(contents.to_string());
    Ok(())
}

fn run_prompt() {
    let stdin: io::Stdin = io::stdin();
    let mut reader: io::StdinLock<'_> = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line: String = String::new();
        let bytes_read: usize = reader.read_line(&mut line).expect("Failed to read line");

        if bytes_read == 0 {
            break; // End of input
        }

        run(line.trim().to_string());
    }
}

fn run(source: String) {
    let mut scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in &tokens {
        println!("{token}");
    }
}

pub fn error(line: usize, msg: &str) -> ! {
    report(line, "", msg)
}

fn report(line: usize, where_: &str, msg: &str) -> ! {
    panic!("[line {line}] Error {where_}: {msg}.")
}

