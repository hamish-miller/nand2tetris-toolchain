/// Translates VM commands into Hack assembly code.

use std::fs::File;
use std::io::{prelude::*, BufWriter};

use crate::parser::CommandType;

const VERBOSE: bool = true;

fn arithmetic_binary(op: &str) -> Vec<&str> {
    vec!("@SP", "A=M-1", "D=M", "A=A-1", op, "@SP", "M=M-1")
}

fn arithmetic_unary(op: &str) -> Vec<&str> {
    vec!("@SP", "A=M-1", op)
}

fn logical_binary<'a>(cond: &'static str, label: &'a LogicLabel) -> Vec<&'a str> {
    vec!("@SP", "A=M-1", "D=M", "A=A-1", "D=M-D", &label.jump_t, cond,
         "@SP", "A=M-1", "A=A-1", "M=0", &label.jump_f, "0;JMP", &label.dest_t,
         "@SP", "A=M-1", "A=A-1", "M=-1", &label.dest_f,
         "@SP", "M=M-1")
}

pub struct CodeWriter {
    writer: BufWriter<File>,
    label_maker: LabelGenerator,
}

impl CodeWriter {
    #![allow(non_snake_case)]  // Contract pre-specified
    #![allow(unused_must_use)] // Ignore writer.write Result

    pub fn new(file: File) -> Self {
        let mut codewriter = CodeWriter {
            writer: BufWriter::new(file),
            label_maker: LabelGenerator::new(),
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
        let label = self.label_maker.next().unwrap();
        let assembly = match command.as_str() {
            "add" => arithmetic_binary("M=M+D"),
            "sub" => arithmetic_binary("M=M-D"),
            "neg" => arithmetic_unary("M=-M"),
            "eq" => logical_binary("D;JEQ", &label),
            "gt" => logical_binary("D;JGT", &label),
            "lt" => logical_binary("D;JLT", &label),
            "and" => arithmetic_binary("M=M&D"),
            "or" => arithmetic_binary("M=M|D"),
            "not" => arithmetic_unary("M=!M"),
            _ => panic!("Unexpected Arithmetic Command: {}", command),
        };

        if VERBOSE {
            self.writer.write(b"// ");
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
            self.writer.write(b"// ");
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

#[derive(PartialEq)]
struct LogicLabel {
    dest_t: String,
    dest_f: String,
    jump_t: String,
    jump_f: String,
}

impl LogicLabel {
    pub fn new(suffix: usize) -> Self {
        LogicLabel {
            dest_t: format!("(JUMP_TRUE_{})", suffix),
            dest_f: format!("(JUMP_FALSE_{})", suffix),
            jump_t: format!("@JUMP_TRUE_{}", suffix),
            jump_f: format!("@JUMP_FALSE_{}", suffix),
        }
    }
}

struct LabelGenerator {
    count: usize,
}

impl LabelGenerator {
    fn new() -> Self {
        LabelGenerator { count: 0 }
    }
}

impl Iterator for LabelGenerator {
    type Item = LogicLabel;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(LogicLabel::new(self.count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_generator_creates_unique_labels() {
        let mut generator = LabelGenerator::new();

        let label_1 = generator.next();
        let label_2 = generator.next();

        assert!(label_1 != label_2);
    }
}
