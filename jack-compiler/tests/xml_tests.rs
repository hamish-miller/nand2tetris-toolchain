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
            let mut e = CompilationEngine::new(t, &mut w, false);

            e.compile();
            let out = std::str::from_utf8(&w).unwrap();

            for (o, e) in out.lines().zip(xml.lines()) {
                assert_eq!(o, e);
            }
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<subroutineDec>
<keyword>method</keyword>
<keyword>void</keyword>
<identifier>baz</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_parameter_list_empty
"\
class Foo {
    function void bar() {}
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
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
<parameterList>
<keyword>int</keyword>
<identifier>baz</identifier>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
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
<parameterList>
<keyword>char</keyword>
<identifier>baz</identifier>
<symbol>,</symbol>
<keyword>void</keyword>
<identifier>bat</identifier>
<symbol>,</symbol>
<identifier>Bam</identifier>
<identifier>bam</identifier>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
</varDec>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_nop_class_subroutine_variable_declaration_primitive_types
"\
class Foo {
    function void bar() {
        var int baz;
        var char bat;
        var boolean bam;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<keyword>char</keyword>
<identifier>bat</identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<keyword>boolean</keyword>
<identifier>bam</identifier>
<symbol>;</symbol>
</varDec>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>char</keyword>
<identifier>baz</identifier>
<symbol>,</symbol>
<identifier>bam</identifier>
<symbol>,</symbol>
<identifier>bat</identifier>
<symbol>;</symbol>
</varDec>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_let_int_constant
"\
class Foo {
    function void bar() {
        var int baz;
        let baz = 42;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>baz</identifier>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>42</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_let_string_constant
"\
class Foo {
    function void bar() {
        var String baz;
        let baz = \"FooBarBaz\";
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<identifier>String</identifier>
<identifier>baz</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>baz</identifier>
<symbol>=</symbol>
<expression>
<term>
<stringConstant>FooBarBaz</stringConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_let_keyword_constant
"\
class Foo {
    function void bar() {
        var boolean t, f;

        let t = true;
        let f = false;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>boolean</keyword>
<identifier>t</identifier>
<symbol>,</symbol>
<identifier>f</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>t</identifier>
<symbol>=</symbol>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<letStatement>
<keyword>let</keyword>
<identifier>f</identifier>
<symbol>=</symbol>
<expression>
<term>
<keyword>false</keyword>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_let_array
"\
class Foo {
    function void bar() {
        var Array a;
        // let a = Array.new(5);
        let a[0] = 42;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<identifier>Array</identifier>
<identifier>a</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>a</identifier>
<symbol>[</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
<symbol>]</symbol>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>42</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_let_multiple
"\
class Foo {
    function void bar() {
        var int x, y;
        var String s;

        let x = 0;
        let s = \"thirty two\";
        let y = 64;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>x</identifier>
<symbol>,</symbol>
<identifier>y</identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<identifier>String</identifier>
<identifier>s</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>x</identifier>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<letStatement>
<keyword>let</keyword>
<identifier>s</identifier>
<symbol>=</symbol>
<expression>
<term>
<stringConstant>thirty two</stringConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<letStatement>
<keyword>let</keyword>
<identifier>y</identifier>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>64</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_if_literals
"\
class Foo {
    function void bar() {
        if (true) {}
        if (false) {}
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</ifStatement>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<keyword>false</keyword>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</ifStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_if_else
"\
class Foo {
    function void bar() {
        if (true) {
        } else {
        }
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</ifStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_while_infinite
"\
class Foo {
    function void bar() {
        while (true) {}
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<whileStatement>
<keyword>while</keyword>
<symbol>(</symbol>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
</statements>
<symbol>}</symbol>
</whileStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_do_subroutine
"\
class Foo {
    function void bar() {
        do bar();
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<expressionList>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_do_class_subroutine
"\
class Foo {
    function void bar() {
        do Bar.bar();
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>Bar</identifier>
<symbol>.</symbol>
<identifier>bar</identifier>
<symbol>(</symbol>
<expressionList>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_return
"\
class Foo {
    function void bar() {
        return;
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_statements_return_expression
"\
class Foo {
    function int bar() {
        return 42;
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>42</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_expression_single_op
"\
class Foo {
    function int bar() {
        return 9 + 10;
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>9</integerConstant>
</term>
<symbol>+</symbol>
<term>
<integerConstant>10</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_expression_multiple_op
"\
class Foo {
    function int bar() {
        return 1 + 2 - 3 * 4 / 5;
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
<symbol>+</symbol>
<term>
<integerConstant>2</integerConstant>
</term>
<symbol>-</symbol>
<term>
<integerConstant>3</integerConstant>
</term>
<symbol>*</symbol>
<term>
<integerConstant>4</integerConstant>
</term>
<symbol>/</symbol>
<term>
<integerConstant>5</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_expression_op_xml_markup_clash
"\
class Foo {
    function boolean bar() {
        return 1 < 2 & 2 > 1;
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
<keyword>boolean</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
<symbol>&lt;</symbol>
<term>
<integerConstant>2</integerConstant>
</term>
<symbol>&amp;</symbol>
<term>
<integerConstant>2</integerConstant>
</term>
<symbol>&gt;</symbol>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_expression_list
"\
class Foo {
    function void bar() {
        do baz(16, 32, 64);
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
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>baz</identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<integerConstant>16</integerConstant>
</term>
</expression>
<symbol>,</symbol>
<expression>
<term>
<integerConstant>32</integerConstant>
</term>
</expression>
<symbol>,</symbol>
<expression>
<term>
<integerConstant>64</integerConstant>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_integer_constant
"\
class Foo {
    function int bar() { return 42; }
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>42</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_string_constant
"\
class Foo {
    function String bar() { return \"something\"; }
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
<identifier>String</identifier>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<stringConstant>something</stringConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_keyword_constant
"\
class Foo {
    function boolean bar() { return true; }
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
<keyword>boolean</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_variable_name
"\
class Foo {
    function int bar() {
        var int x;
        let x = 1;
        return x;
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>x</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>x</identifier>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<identifier>x</identifier>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_variable_name_array
"\
class Foo {
    function int bar() {
        var Array a;
        return a[0];
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<identifier>Array</identifier>
<identifier>a</identifier>
<symbol>;</symbol>
</varDec>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<identifier>a</identifier>
<symbol>[</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
<symbol>]</symbol>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_embedded_expression
"\
class Foo {
    function int bar() {
        return 2 * (3 + 4);
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>2</integerConstant>
</term>
<symbol>*</symbol>
<term>
<symbol>(</symbol>
<expression>
<term>
<integerConstant>3</integerConstant>
</term>
<symbol>+</symbol>
<term>
<integerConstant>4</integerConstant>
</term>
</expression>
<symbol>)</symbol>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_unary_op
"\
class Foo {
    function int error() {
        return -1;
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
<keyword>int</keyword>
<identifier>error</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<symbol>-</symbol>
<term>
<integerConstant>1</integerConstant>
</term>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_internal_subroutine_call
"\
class Foo {
    function int bar() { return 1; }
    function int baz() { return bar(); }
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<subroutineDec>
<keyword>function</keyword>
<keyword>int</keyword>
<identifier>baz</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<identifier>bar</identifier>
<symbol>(</symbol>
<expressionList>
</expressionList>
<symbol>)</symbol>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);


jack_to_xml_test!(
test_compiler_class_subroutine_term_external_subroutine_call
"\
class Foo {
    function int bar() { return Baz.baz(); }
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
<keyword>int</keyword>
<identifier>bar</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<identifier>Baz</identifier>
<symbol>.</symbol>
<identifier>baz</identifier>
<symbol>(</symbol>
<expressionList>
</expressionList>
<symbol>)</symbol>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<symbol>}</symbol>
</class>
"
);
