/// Executable for translating .vm intermediary to .asm assembly.
///
/// Usage: vm-translator <file.vm | dir>

use std::env;
use std::path::Path;

use translator;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: vm-translator <file.vm | dir>");
        std::process::exit(1);
    });

    let path_vm = Path::new(arg);
    let path_asm = {
        if path_vm.is_dir() {
            path_vm.join(path_vm.file_stem().unwrap())
        } else {
            path_vm.to_path_buf()
        }
    }.with_extension("asm");

    translator::translate(path_vm, &path_asm)
}
