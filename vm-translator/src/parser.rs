/// Handles the parsing of a single .vm file and encapsulates access to the input code.

use std::error;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

pub struct Parser {
    reader: BufReader<File>,
    eof: bool,
    pub line: String,
    tokens: Vec<String>,
}

impl Parser {
    #![allow(non_snake_case)]  // Contract pre-specified

    pub fn new(file: File) -> Self {
        Parser {
            reader: BufReader::new(file),
            eof: false,
            line: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn hasMoreCommands(&self) -> bool {
        !self.eof
    }

    pub fn advance(&mut self) {
        let mut line = String::new();

        if let Ok(0) = self.reader.read_line(&mut line) {
            self.eof = true;
        }

        // Remove Comments
        if let Some(i) = line.find("//") {
            let (instruction, _comment) = line.split_at(i);
            line = String::from(instruction);
        }

        // Remove Whitespace
        self.line = line.trim().to_string();

        // Skip over empty lines
        if self.line.is_empty() && self.hasMoreCommands() {
            self.advance();
        }

        // Split tokens
        self.tokens = self.line.split_whitespace().map(|s| s.to_string()).collect();
    }

    pub fn commandType(&self) -> CommandType {
        self.tokens[0].parse::<CommandType>().unwrap()
    }

    pub fn arg1(&self) -> String {
        // C_ARITHEMETIC commands return themselves
        let arg1 = self.tokens.get(1).or_else(|| { self.tokens.get(0) });

        arg1.expect("Unexpected empty tokens")
            .clone()
    }

    pub fn arg2(&self) -> isize {
        let arg2 = self.tokens.get(2);

        arg2.expect("Too few tokens to parse arg2")
            .parse::<isize>()
            .expect(&format!("Failed to parse int from {}", arg2.unwrap()))
    }
}


#[allow(non_camel_case_types)]
pub enum CommandType {
    // Stage 1: Stack Arithmetic
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    // Stage 2: Memory Access
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}

#[derive(Debug)]
pub struct ParseCommandTypeError {}

impl error::Error for ParseCommandTypeError {}
impl fmt::Display for ParseCommandTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseCommandTypeError")
    }
}

const ARITHMETICS: [&str; 9] = ["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];

impl FromStr for CommandType {
    type Err = ParseCommandTypeError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        use CommandType::*;
        match token {
            "push" => Ok(C_PUSH),
            "pop" => Ok(C_POP),
            "label" => Ok(C_LABEL),
            "goto" => Ok(C_GOTO),
            "if-goto" => Ok(C_IF),
            "function" => Ok(C_FUNCTION),
            "return" => Ok(C_RETURN),
            "call" => Ok(C_CALL),
            _ if ARITHMETICS.contains(&token) => Ok(C_ARITHMETIC),
            _ => Err(ParseCommandTypeError{}),
        }
    }
}
