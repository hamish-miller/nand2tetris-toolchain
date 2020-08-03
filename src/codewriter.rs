/// Translates VM commands into Hack assembly code.

use std::ffi::OsStr;
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

fn push_constant(constant: &str) -> Vec<&str> {
    vec!(constant, "D=A",
         "@SP", "A=M", "M=D",
         "@SP", "M=M+1")
}

fn push_indirect<'a>(segment: &'a str, index: &'a str) -> Vec<&'a str> {
    vec!(segment, "D=M", index, "A=D+A", "D=M",
         "@SP", "A=M", "M=D",
         "@SP", "M=M+1")
}

fn push_direct<'a>(register: &'a str) -> Vec<&'a str> {
    vec!(register, "D=M",
         "@SP", "A=M", "M=D",
         "@SP", "M=M+1")
}

fn push_static(static_label: &str) -> Vec<&str> {
    vec!(static_label, "D=M",
         "@SP", "A=M", "M=D",
         "@SP", "M=M+1")
}

fn pop_indirect<'a>(segment: &'a str, index: &'a str) -> Vec<&'a str> {
    vec!(segment, "D=M", index, "D=D+A", "@R13", "M=D",
         "@SP", "M=M-1", "A=M", "D=M",
         "@R13", "A=M", "M=D")
}

fn pop_direct<'a>(register: &'a str) -> Vec<&'a str> {
    vec!(register, "D=A", "@R13", "M=D",
         "@SP", "M=M-1", "A=M", "D=M",
         "@R13", "A=M", "M=D")
}

fn pop_static(static_label: &str) -> Vec<&str> {
    vec!("@SP", "M=M-1", "A=M", "D=M",
         static_label, "M=D")
}


pub struct CodeWriter {
    writer: BufWriter<File>,
    file_name: String,
    label_maker: LabelGenerator,
}

impl CodeWriter {
    #![allow(non_snake_case)]  // Contract pre-specified
    #![allow(unused_must_use)] // Ignore writer.write Result

    pub fn new(file: File) -> Self {
        let mut codewriter = CodeWriter {
            writer: BufWriter::new(file),
            file_name: String::new(),
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

    pub fn setFileName(&mut self, fileName: &OsStr) {
        self.file_name = fileName.to_os_string().into_string().unwrap()
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
        let i = format!("@{}", index);
        let register: String;
        let static_label: String;

        use CommandType::*;
        let (comment, assembly) = match command {
            C_PUSH => ("push", match segment.as_str() {
                "constant" => push_constant(i.as_str()),
                "local" => push_indirect("@LCL", i.as_str()),
                "argument" => push_indirect("@ARG", i.as_str()),
                "this" => push_indirect("@THIS", i.as_str()),
                "that" => push_indirect("@THAT", i.as_str()),
                "pointer" => {
                    register = format!("@R{}", 3 + index);
                    push_direct(register.as_str())
                },
                "temp" => {
                    register = format!("@R{}", 5 + index);
                    push_direct(register.as_str())
                },
                "static" => {
                    static_label = format!("@{}.{}", self.file_name, index);
                    push_static(static_label.as_str())
                }
                _ => panic!("Unknown segment: {}", segment),
            }),
            C_POP => ("pop", match segment.as_str() {
                "constant" => panic!("pop constant is invalid."),
                "local" => pop_indirect("@LCL", i.as_str()),
                "argument" => pop_indirect("@ARG", i.as_str()),
                "this" => pop_indirect("@THIS", i.as_str()),
                "that" => pop_indirect("@THAT", i.as_str()),
                "pointer" => {
                    register = format!("@R{}", 3 + index);
                    pop_direct(register.as_str())
                },
                "temp" => {
                    register = format!("@R{}", 5 + index);
                    pop_direct(register.as_str())
                },
                "static" => {
                    static_label = format!("@{}.{}", self.file_name, index);
                    pop_static(static_label.as_str())
                }
                _ => panic!("Unknown segment: {}", segment),
            }),
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

    pub fn writeLabel(&mut self, label: String) {
        if VERBOSE { self.writer.write(b"// label\n"); };
        self.writer.write(b"(");
        self.writer.write(label.as_bytes());
        self.writer.write(b")\n");
    }

    pub fn writeIf(&mut self, label: String) {
        if VERBOSE { self.writer.write(b"// if-goto\n"); };

        let l = format!("@{}", label);
        let assembly = vec!("@SP", "M=M-1", "A=M", "D=M", l.as_str(), "D;JNE");

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
