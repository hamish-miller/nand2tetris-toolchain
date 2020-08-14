/// Parser: Encapsulate access to input code.

use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Parser {
    reader: BufReader<File>,
    eof: bool,
    pub line: String,
}

impl Parser {
    pub fn new(file: File) -> Self {
        Parser {
            reader: BufReader::new(file),
            eof: false,
            line: String::new(),
        }
    }

    #[allow(non_snake_case)]
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
    }

    #[allow(non_snake_case)]
    pub fn commandType(&self) -> CommandType {
        if self.line.starts_with("@") {
            return CommandType::A_COMMAND
        }

        if self.line.starts_with("(") && self.line.ends_with(")") {
            return CommandType::L_COMMAND
        }

        CommandType::C_COMMAND
    }

    pub fn symbol(&self) -> String {
        String::from(self.line[1..].replacen(")", "", 1))
    }

    pub fn dest(&self) -> String {
        let mut dest = self.line.clone();

        if let Some(i) = dest.find("=") {
            dest = dest.split_at(i).0.to_string();
        }

        dest
    }

    pub fn comp(&self) -> String {
        let mut comp = self.line.clone();

        if let Some(i) = comp.find("=") {
            comp = comp.split_at(i + 1).1.to_string();
        }

        if let Some(i) = comp.find(";") {
            comp = comp.split_at(i).0.to_string();
        }

        comp
    }

    pub fn jump(&self) -> String {
        let mut jump = self.line.clone();

        if let Some(i) = jump.find(";") {
            jump = jump.split_at(i + 1).1.to_string();
        }

        jump
    }
}


#[allow(non_camel_case_types)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}
