/// CompilationEngine: Compiles token stream via recursive descent.

use std::io::Write;

use crate::tokenizer::{Token, TokenStream};

pub struct CompilationEngine<T: TokenStream, W: Write> {
    tokens: T,
    writer: W,
}

impl<T, W> CompilationEngine<T, W> where T: TokenStream, W: Write {
    #![allow(non_snake_case)]  // Contract pre-specified
    #![allow(unused_must_use)] // Ignore writeln! Result

    pub fn new(src: T, dst: W) -> Self {
        CompilationEngine {
            tokens: src,
            writer: dst,
        }
    }

    pub fn compile(&mut self) {
        self.compileClass();
    }

    fn writeKeyword(&mut self) {
        if let Some(Token::Keyword(k)) = self.tokens.next() {
            writeln!(self.writer, "<keyword>{}</keyword>", k.0);
        }
    }

    fn writeIdentifier(&mut self) {
        if let Some(Token::Identifier(i)) = self.tokens.next() {
            writeln!(self.writer, "<identifier>{}</identifier>", i.0);
        }
    }

    fn writeSymbol(&mut self) {
        if let Some(Token::Symbol(s)) = self.tokens.next() {
            writeln!(self.writer, "<symbol>{}</symbol>", s.0);
        }
    }

    /// 'class' className '{' classVarDec* subroutineDec* '}'
    fn compileClass(&mut self) {
        writeln!(self.writer, "{}", "<class>");
        self.writeKeyword();
        self.writeIdentifier();
        self.writeSymbol();

        self.compileClassVarDec();
        self.compileSubroutine();

        self.writeSymbol();
        writeln!(self.writer, "{}", "</class>");
    }

    /// ('static' | 'field') type varName (',' varName)* ';'
    fn compileClassVarDec(&mut self) {
    }

    /// ('constructor' | 'function' | 'method')
    /// ('void' | type) subroutineName '(' parameterList ')'
    /// subRoutineBody
    fn compileSubroutine(&mut self) {
    }
}
