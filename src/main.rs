/// Virtual Machine for generating .asm assembly from .vm bytecode.
///
/// Usage: hack-virtual-machine <file.vm | dir>

use std::env;
use std::ffi::{OsStr, OsString};
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
        println!("Usage: hack-virtual-machine <file.vm | dir>");
        std::process::exit(1);
    });

    let vm_path = Path::new(arg);
    let asm_path = {
        if vm_path.is_dir() {
            vm_path.join(vm_path.file_stem().unwrap())
        } else {
            vm_path.to_path_buf()
        }.with_extension("asm")
    };
    let asm = fs::File::create(asm_path)?;
    let mut codewriter = CodeWriter::new(asm);

    // Handle single .vm and directory with multiple .vm files
    type VmFiles = Vec<(OsString, fs::File)>;
    let vms: VmFiles = match vm_path {
        _ if vm_path.is_dir() => {
            let (sys_vm, vms): (VmFiles, VmFiles) = {
                vm_path.read_dir()?
                       .filter_map(Result::ok)
                       .filter_map(|de| {
                           let path = de.path();
                           let ext = Some(OsStr::new("vm"));

                           if !(path.is_file() && path.extension() == ext) {
                               return None
                           }

                           let name = de.file_name();
                           let file = fs::File::open(path).unwrap();
                           Some((name, file))
                       })
                       .partition(|(name, _file)| { name == "Sys.vm" })
            };

            // Partition necessary to move Sys.vm to front
            sys_vm.into_iter().chain(vms.into_iter()).collect()
        },
        _ if vm_path.is_file() => {
            let name = vm_path.file_name().unwrap().to_os_string();
            let file = fs::File::open(vm_path)?;
            vec!((name, file))
        },
        _ => panic!("Not a file or directory."),
    };

    for (name, file) in vms.into_iter() {
        let mut parser = Parser::new(file);
        parser.advance();
        codewriter.setFileName(&name);

        while parser.hasMoreCommands() {
            use CommandType::*;
            match parser.commandType() {
                C_ARITHMETIC => codewriter.writeArithmetic(parser.arg1()),
                C_PUSH       => codewriter.writePushPop(C_PUSH, parser.arg1(), parser.arg2()),
                C_POP        => codewriter.writePushPop(C_POP, parser.arg1(), parser.arg2()),
                C_LABEL      => codewriter.writeLabel(parser.arg1()),
                C_GOTO       => codewriter.writeGoto(parser.arg1()),
                C_IF         => codewriter.writeIf(parser.arg1()),
                C_FUNCTION   => codewriter.writeFunction(parser.arg1(), parser.arg2()),
                C_RETURN     => codewriter.writeReturn(),
                C_CALL       => codewriter.writeCall(parser.arg1(), parser.arg2()),
            }

            parser.advance();
        }
    }

    codewriter.close();

    Ok(())
}
