/// JackCompiler: Executable for compiling .jack source to .vm intermediate.
///
/// Usage: jack-compiler <file.jack | dir>

use std::env;
use std::path::Path;

use compiler;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: jack-compiler <file.jack | dir>");
        std::process::exit(1);
    });

    let targets = compiler::targets(Path::new(arg));

    for (source, target) in targets.iter() {
        println!("Compiling {}", source.as_path().to_string_lossy());
        compiler::compile(&source, &target)?;
    }

    Ok(())
}
