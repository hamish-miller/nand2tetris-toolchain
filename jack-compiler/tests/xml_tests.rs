/// Tests!

use compiler::tokenizer::JackTokenizer;
use compiler::engine::CompilationEngine;


macro_rules! jack_to_xml_test {
    ($name:tt $jack:tt -> $xml:tt) => {
        #[test]
        fn $name() {
            let jack = $jack;
            let xml = $xml;

            let t = JackTokenizer::new(&jack);
            let mut w = Vec::new();
            let mut e = CompilationEngine::new(t, &mut w);

            e.compile();
            let out = std::str::from_utf8(&w).unwrap();

            assert_eq!(out, xml);
        }
    }
}


jack_to_xml_test!(
test_compiler_empty_class
"class Foo {}"
->
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_pod_class_static_field
"\
class Foo {
    static int bar;
    field int baz;
}
"
->
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
"
);


jack_to_xml_test!(
test_compiler_pod_class_primitive_types
"\
class Foo {
    field int bar;
    field boolean baz;
    field char bat;
}
"
->
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
"
);


jack_to_xml_test!(
test_compiler_pod_class_non_primitive_types
"\
class Foo {
    field Bar bar;
}
"
->
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
"
);


jack_to_xml_test!(
test_compiler_pod_class_multiple_variable_declaration
"\
class Foo {
    field int bar, baz, bat;
}
"
->
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
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_variants
"\
class Foo {
    constructor Foo new() {}
    function void bar() {}
    method void baz() {}
}
"
->
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
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_parameter_list_single
"\
class Foo {
    function void bar(int baz) {}
}
"
->
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<symbol>}</symbol>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_parameter_list_multiple
"\
class Foo {
    function void bar(char baz, void bat, Bam bam) {}
}
"
->
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<keyword>char</keyword>
<identifier>baz</identifier>
<symbol>,</symbol>
<keyword>void</keyword>
<identifier>bat</identifier>
<symbol>,</symbol>
<identifier>Bam</identifier>
<identifier>bam</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<symbol>}</symbol>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_variable_declaration_single
"\
class Foo {
    function void bar() {
        var int baz;
    }
}
"
->
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
<symbol>}</symbol>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_variable_declaration_multiple
"\
class Foo {
    function void bar() {
        var char baz, bam, bat;
    }
}
"
->
"\
<class>
<keyword>class</keyword>
<identifier>Foo</identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>var</keyword>
<keyword>char</keyword>
<identifier>baz</identifier>
<symbol>,</symbol>
<identifier>bam</identifier>
<symbol>,</symbol>
<identifier>bat</identifier>
<symbol>;</symbol>
<symbol>}</symbol>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);
