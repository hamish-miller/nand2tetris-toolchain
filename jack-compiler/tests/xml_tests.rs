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


#[test]
fn test_compiler_pod_class_static_field() {
    let jack =
"\
class Foo {
    static int bar;
    field int baz;
}
";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>static</keyword>
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>;</symbol>
</classVarDec>
<classVarDec>
<keyword>field</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
</classVarDec>
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


#[test]
fn test_compiler_pod_class_primitive_types() {
    let jack =
"\
class Foo {
    field int bar;
    field boolean baz;
    field char bat;
}
";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>field</keyword>
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>;</symbol>
</classVarDec>
<classVarDec>
<keyword>field</keyword>
<keyword>boolean</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
</classVarDec>
<classVarDec>
<keyword>field</keyword>
<keyword>char</keyword>
<identifier>bat</identifier>
<symbol>;</symbol>
</classVarDec>
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


#[test]
fn test_compiler_pod_class_non_primitive_types() {
    let jack =
"\
class Foo {
    field Bar bar;
}
";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>field</keyword>
<identifier>Bar</identifier>
<identifier>bar</identifier>
<symbol>;</symbol>
</classVarDec>
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


#[test]
fn test_compiler_pod_class_multiple_variable_declaration() {
    let jack =
"\
class Foo {
    field int bar, baz, bat;
}
";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>field</keyword>
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>,</symbol>
<identifier>baz</identifier>
<symbol>,</symbol>
<identifier>bat</identifier>
<symbol>;</symbol>
</classVarDec>
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


#[test]
fn test_compiler_nop_class_subroutine_variants() {
    let jack =
"\
class Foo {
    constructor Foo new() {}
    function void bar() {}
    method void baz() {}
}
";
    let xml =
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>constructor</keyword>
<identifier>Foo</identifier>
<identifier>new</identifier>
<symbol>(</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<symbol>}</symbol>
</subroutineDec>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<symbol>}</symbol>
</subroutineDec>
<subroutineDec>
<keyword>method</keyword>
<keyword>void</keyword>
<identifier>baz</identifier>
<symbol>(</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<symbol>}</symbol>
</subroutineDec>
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
