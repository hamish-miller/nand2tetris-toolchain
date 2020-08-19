/// Compiler: Library for compiling .jack source to .xml / .vm intermediates.

use std::path::Path;

mod tokenizer;

use tokenizer::JackTokenizer;

pub fn analyze(src_jack: &Path, _dst_xml: &Path) -> Result<(), std::io::Error> {
    let tokenizer = JackTokenizer::new(src_jack);

    for token in tokenizer {
        println!("{:?}", token);
    }

    Ok(())
}
