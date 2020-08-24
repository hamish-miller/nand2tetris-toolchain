/// Compiler: Library for compiling .jack source to .xml / .vm intermediates.

use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};

pub mod tokenizer;
pub mod engine;

use tokenizer::JackTokenizer;
use engine::CompilationEngine;

pub fn analyze(src_jack: &Path, dst_xml: &Path) -> Result<(), std::io::Error> {
    let tokenizer = JackTokenizer::open(src_jack);
    let file_xml = File::create(dst_xml)?;
    let mut engine = CompilationEngine::new(tokenizer, file_xml, true);

    engine.compile();
    Ok(())
}


#[allow(unused)]
pub fn compile(src_jack: &Path, dst_xml: &Path) -> Result<(), std::io::Error> {
    Ok(())
}


pub fn targets(path: &Path) -> Vec<(PathBuf, PathBuf)> {
    match path {
        p if p.is_dir() => {
            p.read_dir()
             .expect("Failed to read directory.")
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
    }
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
