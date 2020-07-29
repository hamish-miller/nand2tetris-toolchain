/// Translates VM commands inot Hack assembly code.

use std::fs::File;
use std::io::{prelude::*, BufWriter};

use crate::parser::CommandType;

const VERBOSE: bool = true;

#[allow(dead_code)]
const ADD: [&'static str; 12] = ["@SP", "M=M-1", "@SP", "A=M", "D=M", "@SP", "M=M-1", "@SP", "A=M", "M=M+D", "@SP", "M=M+1"];
const ADD_OPTIMISED: [&'static str; 7] = ["@SP", "A=M-1", "D=M", "A=A-1", "M=M+D", "@SP", "M=M-1"];

pub struct CodeWriter {
    writer: BufWriter<File>,
}

impl CodeWriter {
    #![allow(non_snake_case)]  // Contract pre-specified
    #![allow(unused_must_use)] // Ignore writer.write Result

    pub fn new(file: File) -> Self {
        let mut codewriter = CodeWriter {
            writer: BufWriter::new(file),
        };

        // Initiate stack pointer
        let stack_init = vec!("// stack init", "@256", "D=A", "@SP", "M=D");

        for line in stack_init.iter() {
            codewriter.writer.write(line.as_bytes());
            codewriter.writer.write(b"\n");
        }

        codewriter
    }

    #[allow(dead_code)]
    pub fn setFileName(&mut self, _fileName: String) {
        unimplemented!("Multiple .vm files not yet supported.");
    }

    pub fn writeArithmetic(&mut self, command: String) {
        let assembly = match command.as_str() {
            // SimpleAdd
            "add" => ADD_OPTIMISED,
            // StackTest
            "sub" => unimplemented!(),
            "neg" => unimplemented!(),
            "eq" => unimplemented!(),
            "gt" => unimplemented!(),
            "lt" => unimplemented!(),
            "and" => unimplemented!(),
            "or" => unimplemented!(),
            "not" => unimplemented!(),
            _ => panic!("Unexpected Arithmetic Command: {}", command),
        };

        if VERBOSE {
            self.writer.write(b"\\\\");
            self.writer.write(command.as_bytes());
            self.writer.write(b"\n");
        }

        for line in assembly.iter() {
            self.writer.write(line.as_bytes());
            self.writer.write(b"\n");
        }
    }

    pub fn writePushPop(&mut self, command: CommandType, segment: String, index: isize) {
        assert!(segment == "constant", "Other segments not yet implemented.");
        let a_index = format!("@{}", index);

        use CommandType::*;
        let (comment, assembly) = match command {
            C_PUSH => ("push", vec!(a_index.as_str(), "D=A",
                                    "@SP", "A=M", "M=D",
                                    "@SP", "M=M+1")),
            C_POP => unimplemented!("Stage 2: Memory Access"),
            _ => panic!("Unexpected CommandType."),
        };

        if VERBOSE {
            self.writer.write(b"\\\\");
            self.writer.write(comment.as_bytes());
            self.writer.write(b"\n");
        }

        for line in assembly.iter() {
            self.writer.write(line.as_bytes());
            self.writer.write(b"\n");
        }
    }

    // Move self prevents use after move
    pub fn close(self) {
    }
}
