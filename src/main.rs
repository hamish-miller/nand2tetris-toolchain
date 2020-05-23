/// Assembler for assembling .hack binaries from .asm assembly.
///
/// Usage: hack-assembler <file.asm>

use std::env;
use std::fs;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

mod code;
mod parser;

use parser::{Parser, CommandType};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: hack-assembler <file.asm>");
        std::process::exit(1);
    });

    let assembly_path = Path::new(arg);
    let assembly = fs::File::open(assembly_path)
        .expect("Failed to read file.");

    let hack_path = assembly_path.with_extension("hack");
    let hack = fs::File::create(hack_path.file_name().unwrap())
        .expect("Failed to create file.");

    let mut parser = Parser::new(assembly);
    let mut writer = BufWriter::new(hack);

    parser.advance();

    while parser.hasMoreCommands() {
        let binary = match parser.commandType() {
            CommandType::A_COMMAND => {
                format!("{:0>16b}", parser.symbol().parse::<i32>().unwrap())
            },
            CommandType::C_COMMAND => {
                format!("111{}{}{}", code::comp(&parser.comp()),
                                     code::dest(&parser.dest()),
                                     code::jump(&parser.jump()),
            )}
        };

        // println!("{:<5} -> {}", parser.line, binary);

        writer.write(binary.as_bytes())?;
        writer.write("\n".as_bytes())?;

        parser.advance();
    }

    writer.flush()?;

    Ok(())
}
