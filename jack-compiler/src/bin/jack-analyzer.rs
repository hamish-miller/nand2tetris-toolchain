/// JackAnalyzer: Executable for compile .jack source to .xml intermediate.

use std::env;
use std::path::Path;

use compiler;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: vm-translator <file.vm | dir>");
        std::process::exit(1);
    });

    // TODO: Multiple .jack
    let path_jack = Path::new(arg);
    let path_xml = path_jack.with_extension("xml");

    compiler::analyze(path_jack, &path_xml)
}
