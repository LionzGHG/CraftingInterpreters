
// pretty-print error messages to the user through the console


use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use crate::lexer::tokens::{Token, TokenType};
use crate::util::print_formatter::StringFormat;

use super::Value;

#[derive(Clone, Debug)]
pub enum ErrorKind {
    NumberOperand(Token),
    UnkownToken(Token),
    UnexpectedToken(Token, &'static str),
    UnexpectedType(Token, Value),
    TypeMismatch(Token, String, Vec<String>),
    ImmutableVar(Token, String),
    UndefinedVar(Token),
    Fatal,
}

#[derive(Clone, Debug)]
pub struct ErrorHandler;

impl ErrorHandler {
    pub fn throw(&self, error: ErrorKind) -> ! {
        match error {
            ErrorKind::NumberOperand(token) => self.throw_number_operand_error(token),
            ErrorKind::UnexpectedToken(token, msg) => self.throw_unexpected_token_error(token, msg),
            ErrorKind::UnkownToken(token) => self.throw_unkown_token_error(token),
            ErrorKind::UnexpectedType(token, value) => self.throw_unexpected_type_error(value, token),
            ErrorKind::TypeMismatch(token, input, expected) => self.throw_type_mismatch_error(token, input, expected),
            ErrorKind::ImmutableVar(token, name) => self.throw_immutable_var_error(token, name),
            ErrorKind::UndefinedVar(token) => self.throw_undefined_var_error(token),
            ErrorKind::Fatal => self.throw_fatal_error()
        }
    }  

    fn extract_line_of_code(&self, file_path: &str, line_number: usize) -> Result<String, String> {
        let path = Path::new(file_path);
        let file = File::open(&path).map_err(|_| "Failed to open the file.")?;

        let reader = io::BufReader::new(file);

        if let Some(line) = reader.lines().enumerate().find(|(i, _)| *i == line_number - 1) {
            let line = line.1.map_err(|_| "Row number out of range.")?;
            return Ok(line);
        }

        Err("Line number out of range.".to_string())
    }

    pub fn default_error_design(
        &self,
        mut title: &str, 
        line: usize, 
        row: usize, 
        code: &str, 
        mut description: &str, 
        help: Option<&str>, 
        note: Option<&str>) -> ! 
    {
        let mut error: String = String::new();

        let file_path: String = String::from("src/Main.rogue");

        error.push_str(format!("{}{} {}\n", "Error".red().bold(), ":".bold(), title.bold()).as_str());
        error.push_str(format!("    {} {}:{}:{}\n", "-->".bold().blue(), file_path, line, row).as_str());
        error.push_str(format!("     {}\n", "|".bold().blue()).as_str());
        
        let len: usize = line.to_string().len();
        
        let line_of_code: String = self.extract_line_of_code("./Main.rogue", line).unwrap();

        let mut line_str: String = format!("{}", line.clone());

        match len {
            1 => error.push_str(format!("{}    {}", line_str.bold().blue(), "|".bold().blue()).as_str()),
            2 => error.push_str(format!("{}   {}", line_str.bold().blue(), "|".bold().blue()).as_str()),
            3 => error.push_str(format!("{}  {}", line_str.bold().blue(), "|".bold().blue()).as_str()),
            4 => error.push_str(format!("{} {}", line_str.bold().blue(), "|".bold().blue()).as_str()),
            _ => panic!("Dude, your file is way too long: {len}"),
        }
        error.push_str(format!("          {}\n", line_of_code).as_str());
        error.push_str(format!("     {}         ", "|".blue().bold()).as_str());
        
        for _ in 0..row {
            error.push_str(" ");
        } 

        error.push_str(format!("{} {}\n", "^".bold().red(), description.style(crate::util::print_formatter::Style::Italic)).as_str());

        error.push_str(format!("     {}\n", "|".bold().blue()).as_str());

        if let Some(help) = help {
            error.push_str(format!("     {} {} {}\n", "=".bold().blue(), "help:".bold(), help).as_str());
        }

        if let Some(note) = note {
            error.push_str(format!("     {} {} {}\n", "=".bold().blue(), "note:".bold(), note).as_str());
        }

        panic!("{}", error);
    }

    fn throw_number_operand_error(&self, token: Token) -> ! {
        self.default_error_design("Wrong number-operand order", token.line, token.row, token.lexeme.as_str(), "Expected number after Operand in Expression", None, None);
    }

    fn throw_unkown_token_error(&self, token: Token) -> ! {
        self.default_error_design("Unexpected Token", token.line, token.row, &token.lexeme, "Unexpected token found here", Some("Remove this token."), None);
    }

    fn throw_unexpected_token_error(&self, token: Token, msg: &str) -> ! {
        self.default_error_design("Unexpected Token", token.line, token.row, &token.lexeme, msg, None, None);
    }

    fn throw_unexpected_type_error(&self, value: Value, token: Token) -> ! {
        self.default_error_design("Unexpected Type", token.line, token.row, &token.lexeme, format!("Expected type `f64`, got value of `{}`", value).as_str(), Some("Change to type `f64`."), None);
    }

    fn throw_type_mismatch_error(&self, token: Token, input: String, expected: Vec<String>) -> ! {
        let mut expected_types: String = String::new();
        
        for type_ in expected.clone() {
            if expected.last().unwrap().eq(&type_) {
                expected_types.push_str(format!(" `{type_}`").as_str());
            } else {
                expected_types.push_str(format!("`{type_}` or").as_str());
            }
        }
        
        self.default_error_design("Type mismatch", token.line, token.row, &token.lexeme, format!("got: `{}`, expected: {}", input, expected_types).as_str(), None, None);
    }

    fn throw_immutable_var_error(&self, token: Token, name: String) -> ! {
        let msg: String = String::from(format!("Cannot assign to `{name}`, because `{name}` is immutable.").as_str());
        let help: String = String::from(format!("Make `{name}` mutable by adding the `mut` keyword.").as_str());
        let note: String = String::from(format!("Variables need to be mutable to be reassigned.").as_str());

        self.default_error_design("Cannot assign to immutable data", token.line, token.row, &token.lexeme, msg.as_str(), Some(help.as_str()), Some(note.as_str()));
    }

    fn throw_undefined_var_error(&self, token: Token) -> ! {
        let msg: String = String::from(format!("Variable `{}` is undefined in this scope.", token.lexeme).as_str());
        let help: String = String::from(format!("Maybe `{}` was moved to another scope or never declared?", token.lexeme).as_str());

        self.default_error_design("Undefined Variable", token.line, token.row, &token.lexeme, msg.as_str(), Some(help.as_str()), None);
    }

    fn throw_fatal_error(&self) -> ! {
        self.default_error_design("Fatal Error", 0, 0, "", "", Some("Try recompiling the program"), Some("Contact support under will.help@gmail.com."));
    }
}

/*
Error: *error title*
    --> file\path\main.rsc:line:row
     |
line | <<literal line of code the error occoured in>>
     |                            ^^^^^ error description
     = help: maybe this will fix it?
     = note: maybe you didnt know this 
Program didnt compile successfully!
*/
#[test]
#[should_panic]
fn test_errors() {
    let mut err_fmt: ErrorHandler = ErrorHandler;

    err_fmt.default_error_design(
        "Value assigned to `initializer` is never read", 
        200, 
        13, 
        "initializer = Value(Var::inferred(true, this.peek(), Null));", 
        "useless variable.", 
        Some("maybe it is overwritten before being read?"), 
        Some("`@warn(error = \"unused_assignments\")` is enabled by default.")
    );
}

