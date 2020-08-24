/// JackAnalyzer: Executable for compile .jack source to .xml intermediate.

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use compiler;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // Handle args.len(0). Ignore args[2..]
    let arg = args.get(1).unwrap_or_else(|| {
        println!("Usage: vm-translator <file.vm | dir>");
        std::process::exit(1);
    });

    let targets: Vec<(PathBuf, PathBuf)> = match Path::new(arg) {
        p if p.is_dir() => {
            p.read_dir()?
             .filter_map(Result::ok)
             .map(|de| de.path())
             .filter(|path| path.is_jack())
             .map(|jack| {let xml = jack.with_extension("xml"); (jack, xml)})
             .collect()
        },
        p if p.is_file() && p.is_jack() => {
            vec!((p.to_path_buf(), p.with_extension("xml")))
        },
        p => {
            dbg!(p, p.exists(), p.is_dir(), p.is_file(), p.is_jack());
            dbg!(std::env::current_dir().unwrap());
            vec!()
        },
    };

    for (source, target) in targets.iter() {
        println!("Compiling {}", source.as_path().to_string_lossy());
        compiler::analyze(&source, &target)?;
    }

    Ok(())
}


// Trait instead of function for uniform call
trait IsJack {
    fn is_jack(&self) -> bool;
}

impl IsJack for Path {
    fn is_jack(&self) -> bool {
        self.extension() == Some(OsStr::new("jack"))
    }
}

