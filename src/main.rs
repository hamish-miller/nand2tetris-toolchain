/// Virtual Machine for generating .asm assembly from .vm bytecode.
///
/// Usage: hack-virtual-machine <file.vm>

use std::env;
use std::fs;
use std::path::Path;

mod codewriter;
mod parser;

use codewriter::CodeWriter;
use parser::{Parser, CommandType};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: hack-virtual-machine <file.vm>");
        std::process::exit(1);
    });

    let vm_path = Path::new(arg);
    let asm_path = vm_path.with_extension("asm");

    let vm = fs::File::open(vm_path)?;
    let asm = fs::File::create(asm_path)?;

    let mut parser = Parser::new(vm);
    let mut codewriter = CodeWriter::new(asm);

    codewriter.setFileName(vm_path.file_stem().unwrap());
    parser.advance();

    while parser.hasMoreCommands() {
        use CommandType::*;
        match parser.commandType() {
            C_ARITHMETIC => codewriter.writeArithmetic(parser.arg1()),
            C_PUSH => codewriter.writePushPop(C_PUSH, parser.arg1(), parser.arg2()),
            C_POP => codewriter.writePushPop(C_POP, parser.arg1(), parser.arg2()),
            C_LABEL => codewriter.writeLabel(parser.arg1()),
            C_GOTO => codewriter.writeGoto(parser.arg1()),
            C_IF => codewriter.writeIf(parser.arg1()),
            _ => unimplemented!("Stage 2: Memory Access.")
        }

        parser.advance();
    }

    codewriter.close();

    Ok(())
}
