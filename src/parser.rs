/// Handles the parsing of a single .vm file and encapsulates access to the input code.

use std::fs::File;
use std::io::{prelude::*, BufReader};

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
        use CommandType::*;

        match self.tokens[0].as_str() {
            "push" => C_PUSH,
            "pop" => C_POP,
            _ => C_ARITHMETIC,
        }
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


#[allow(non_camel_case_types, dead_code)]
pub enum CommandType {
    // Stage 1: Stack Arithmetic
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    // Stage 2: Memory Access
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}
