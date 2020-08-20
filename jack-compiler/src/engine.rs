/// CompilationEngine: Compiles token stream via recursive descent.

use std::io::Write;

use crate::tokenizer::{Token, TokenStream};

pub struct CompilationEngine<T: TokenStream, W: Write> {
    tokens: T,
    writer: W,
    cache: Option<Token>,
}

impl<T, W> CompilationEngine<T, W> where T: TokenStream, W: Write {
    #![allow(non_snake_case)]  // Contract pre-specified
    #![allow(unused_must_use)] // Ignore writeln! Result

    pub fn new(src: T, dst: W) -> Self {
        CompilationEngine {
            tokens: src,
            writer: dst,
            cache: None,
        }
    }

    pub fn compile(&mut self) {
        self.compileClass();
    }

    fn token(&mut self) -> Token {
        if self.cache.is_some() {
            return self.cache.take().unwrap()
        }

        match self.tokens.next() {
            Some(t) => t,
            None => panic!("TokenStream exhausted!"),
        }
    }

    // Unable to parse token -> trust someone else can
    fn cache(&mut self, t: Token) {
        self.cache = Some(t);
    }

    fn openNonTerminal(&mut self, nt: &str) {
        writeln!(self.writer, "<{}>", nt);
    }

    fn closeNonTerminal(&mut self, nt: &str) {
        writeln!(self.writer, "</{}>", nt);
    }

    fn writeKeyword(&mut self, k: &str) {
        writeln!(self.writer, "<keyword>{}</keyword>", k);
    }

    fn writeSymbol(&mut self, s: char) {
        let t = self.token();
        match t.symbol() {
            Some(c) if *c == s => {
                writeln!(self.writer, "<symbol>{}</symbol>", s);
            },
            _ => self.cache(t),
        }
    }

    fn writeIdentifier(&mut self) {
        match self.token().identifier() {
            Some(i) => {
                writeln!(self.writer, "<identifier>{}</identifier>", i);
            },
            _ => {},
        }
    }

    fn writeType(&mut self) {
        const TYPES: [&str; 3] = ["int", "char", "boolean"];
        let t = self.token();
        match t.keyword() {
            Some(k) if TYPES.contains(&k) => self.writeKeyword(k),
            _ => match t.identifier() {
               Some(_) => { self.cache(t); self.writeIdentifier(); },
               _ => self.cache(t),
            }
        }
    }

    /// 'class' className '{' classVarDec* subroutineDec* '}'
    fn compileClass(&mut self) {
        match self.token().keyword() {
            Some("class") => {
                self.openNonTerminal("class");
                self.writeKeyword("class");
                self.writeIdentifier();
                self.writeSymbol('{');

                loop {
                    self.compileClassVarDec();
                    if self.cache.is_some() { break }
                }

                loop {
                    self.compileSubroutine();
                    if self.cache.is_some() { break }
                }

                self.writeSymbol('}');
                self.closeNonTerminal("class");
            },
            _ => panic!("Expected keyword 'class'"),
        }
    }

    /// ('static' | 'field') type varName (',' varName)* ';'
    fn compileClassVarDec(&mut self) {
        const KEYWORDS: [&str; 2] = ["static", "field"];
        let t = self.token();
        match t.keyword() {
            Some(s) if KEYWORDS.contains(&s) => {
                self.openNonTerminal("classVarDec");
                self.writeKeyword(s);

                self.writeType();
                self.writeIdentifier();

                loop {
                    self.writeSymbol(',');
                    if self.cache.is_some() { break }
                    self.writeIdentifier();
                    if self.cache.is_some() { break }
                }

                self.writeSymbol(';');
                self.closeNonTerminal("classVarDec");
            },
            _ => self.cache(t),
        }
    }

    /// ('constructor' | 'function' | 'method')
    /// ('void' | type) subroutineName '(' parameterList ')'
    /// subRoutineBody
    fn compileSubroutine(&mut self) {
        const KEYWORDS: [&str; 3] = ["constructor", "function", "method"];
        let t = self.token();
        match t.keyword() {
            Some(s) if KEYWORDS.contains(&s) => {
                self.openNonTerminal("subroutineDec");
                //
                self.closeNonTerminal("subroutineDec");
            },
            _ => self.cache(t),
        }
    }
}
