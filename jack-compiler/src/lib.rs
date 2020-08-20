/// Compiler: Library for compiling .jack source to .xml / .vm intermediates.

use std::fs::File;
use std::path::Path;

pub mod tokenizer;
pub mod engine;

use tokenizer::JackTokenizer;
use engine::CompilationEngine;

pub fn analyze(src_jack: &Path, dst_xml: &Path) -> Result<(), std::io::Error> {
    let tokenizer = JackTokenizer::open(src_jack);
    let file_xml = File::open(dst_xml)?;
    let mut engine = CompilationEngine::new(tokenizer, file_xml);

    engine.compile();
    Ok(())
}
