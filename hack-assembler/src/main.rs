/// Executable for assembling .hack binaries from .asm assembly.
///
/// Usage: hack-assembler <file.asm>

use std::env;
use std::path::Path;

use assembler;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: hack-assembler <file.asm>");
        std::process::exit(1);
    });

    let path_asm = Path::new(arg);
    let path_hack = path_asm.with_extension("hack");

    assembler::assemble(path_asm, &path_hack)
}
