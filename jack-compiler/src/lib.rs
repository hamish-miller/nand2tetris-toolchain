/// Compiler: Library for compiling .jack source to .xml / .vm intermediates.

use std::path::Path;

mod tokenizer;

use tokenizer::JackTokenizer;

pub fn analyze(src_jack: &Path, _dst_xml: &Path) -> Result<(), std::io::Error> {
    let mut tokenizer = JackTokenizer::new(src_jack);
    tokenizer.advance();

    while tokenizer.hasMoreTokens() {
        tokenizer.advance();

        if tokenizer.token.is_some() {
            println!("{:?}", tokenizer.token.as_ref().unwrap());
        }
    }

    Ok(())
}
