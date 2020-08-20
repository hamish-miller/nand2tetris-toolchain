/// Tests!

use compiler::tokenizer::JackTokenizer;
use compiler::engine::CompilationEngine;


#[test]
fn test_compiler_empty_class() {
    let jack = "class Foo {}";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<symbol>}</symbol>
</class>
";

    let t = JackTokenizer::new(&jack);
    let mut w = Vec::new();
    let mut e = CompilationEngine::new(t, &mut w);

    e.compile();
    let out = std::str::from_utf8(&w).unwrap();

    assert_eq!(out, xml);
}
