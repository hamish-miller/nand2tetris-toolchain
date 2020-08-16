/// Assembler: Library for assembling .hack binaries from .asm assembly.

use std::fs;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

mod code;
mod parser;
mod symboltable;

use parser::{Parser, CommandType};
use symboltable::SymbolTable;

pub fn assemble(src_asm: &Path, dst_hack: &Path) -> Result<(), std::io::Error> {
    let mut symbol_table = SymbolTable::new();

    // First Pass
    let assembly = fs::File::open(src_asm)?;

    let mut parser = Parser::new(assembly);
    let mut rom_address = 0;

    parser.advance();

    while parser.hasMoreCommands() {
        match parser.commandType() {
            CommandType::L_COMMAND => {
                symbol_table.addEntry(&parser.symbol(), rom_address)
            }
            _ => rom_address += 1,
        }

        parser.advance();
    }

    // Second Pass
    let assembly = fs::File::open(src_asm)?;
    let hack = fs::File::create(dst_hack)?;

    let mut parser = Parser::new(assembly);
    let mut writer = BufWriter::new(hack);
    let mut ram_address = SymbolTable::NEXT_AVAILABLE_RAM_ADDRESS;

    parser.advance();

    while parser.hasMoreCommands() {
        let binary = match parser.commandType() {
            CommandType::A_COMMAND => {
                let symbol = parser.symbol();

                let address = match symbol.parse::<u16>() {
                    Ok(address) => address,
                    Err(_) => {
                        if !symbol_table.contains(&symbol) {
                            symbol_table.addEntry(&symbol, ram_address);
                            ram_address += 1;
                        }

                        symbol_table.GetAddress(&symbol)
                    }
                };

                Some(format!("{:0>16b}", address))
            },
            CommandType::C_COMMAND => {
                Some(format!("111{}{}{}", code::comp(&parser.comp()),
                                          code::dest(&parser.dest()),
                                          code::jump(&parser.jump()),
                ))
            },
            CommandType::L_COMMAND => {
                None
            },
        };

        // println!("{:<5} -> {}", parser.line, binary);

        if let Some(binary) = binary {
            writeln!(&mut writer, "{}", binary)?;
        }

        parser.advance();
    }

    writer.flush()?;

    Ok(())
}
