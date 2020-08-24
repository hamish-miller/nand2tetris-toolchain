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

    // keyword, symbol, identifier, integerConstant, stringConstant
    fn writeTerminal(&mut self, tag: &str, value: &str) {
        writeln!(self.writer, "<{}>{}</{}>", tag, value, tag);
    }

    fn writeKeyword(&mut self, k: &str) {
        self.writeTerminal("keyword", k);
    }

    fn writeSymbol(&mut self, s: char) {
        let t = self.token();
        match t.symbol() {
            Some(c) if *c == s => {
                if let Some(s) = match s {
                    '<' => Some("&lt;"),
                    '>' => Some("&gt;"),
                    '&' => Some("&amp;"),
                    _ => None,
                } {
                    self.writeTerminal("symbol", s);
                } else {
                    let s = s.to_string();
                    self.writeTerminal("symbol", &s);
                }
            },
            _ => self.cache(t),
        }
    }

    fn writeIdentifier(&mut self) {
        let t = self.token();
        match t.identifier() {
            Some(i) => self.writeTerminal("identifier", i),
            _ => self.cache(t),
        }
    }

    fn writeIntegerConstant(&mut self) {
        let t = self.token();
        match t.integer_constant() {
            Some(i) => self.writeTerminal("integerConstant", i),
            _ => self.cache(t),
        }
    }

    fn writeStringConstant(&mut self) {
        let t = self.token();
        match t.string_constant() {
            Some(s) => self.writeTerminal("stringConstant", s),
            _ => self.cache(t),
        }
    }

    fn writeKeywordConstant(&mut self) {
        const KEYWORDS: [&str; 4] = ["true", "false", "null", "this"];
        let t = self.token();
        match t.keyword() {
            Some(kc) if KEYWORDS.contains(&kc) => {
                self.writeKeyword(&kc);
            },
            _ => self.cache(t),
        }
    }

    fn writeType(&mut self) {
        const PRIMITIVES: [&str; 4] = ["int", "char", "boolean", "void"];
        let t = self.token();
        match t.keyword() {
            Some(k) if PRIMITIVES.contains(&k) => self.writeKeyword(k),
            _ => match t.identifier() {
               Some(_) => { self.cache(t); self.writeIdentifier(); },
               _ => self.cache(t),
            }
        }
    }

    /// subroutineName '(' 'expressionList' ')' |
    /// (className | varName) '.' subroutineName '(' expressionList ')'
    fn writeSubroutineCall(&mut self) {
        self.writeIdentifier();

        let t = self.token();
        if Some(&'.') ==  t.symbol() {
            self.cache(t);
            self.writeSymbol('.');
            self.writeIdentifier();
        } else {
            self.cache(t);
        }

        self.writeSymbol('(');
        self.compileExpressionList();
        self.writeSymbol(')');
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
                {
                    self.writeKeyword(s);
                    self.writeType();
                    self.writeIdentifier();
                    self.writeSymbol('(');
                    self.compileParameterList();
                    self.writeSymbol(')');

                    self.openNonTerminal("subroutineBody");
                    {
                        self.writeSymbol('{');
                        loop {
                            self.compileVarDec();
                            if self.cache.is_some() { break }
                        }
                        self.compileStatements();
                        self.writeSymbol('}');
                    }
                    self.closeNonTerminal("subroutineBody");
                }
                self.closeNonTerminal("subroutineDec");
            },
            _ => self.cache(t),
        }
    }

    /// ((type varName) (',' type varName)*)?
    fn compileParameterList(&mut self) {
        self.openNonTerminal("parameterList");
        {
            let t = self.token();
            if Some(&')') == t.symbol() {
                self.cache(t);
                self.closeNonTerminal("parameterList");
                return
            } else {
                self.cache(t);
            }

            self.writeType();
            if self.cache.is_some() { return }
            self.writeIdentifier();
            if self.cache.is_some() { return }

            loop {
                self.writeSymbol(',');
                if self.cache.is_some() { break }
                self.writeType();
                if self.cache.is_some() { break }
                self.writeIdentifier();
                if self.cache.is_some() { break }
            }
        }
        self.closeNonTerminal("parameterList");
    }

    /// 'var' type varName (',' varName)* ';'
    fn compileVarDec(&mut self) {
        let t = self.token();
        match t.keyword() {
            Some("var") => {
                self.openNonTerminal("varDec");
                {
                    self.writeKeyword("var");
                    self.writeType();
                    if self.cache.is_some() { return }
                    self.writeIdentifier();
                    if self.cache.is_some() { return }

                    loop {
                        self.writeSymbol(',');
                        if self.cache.is_some() { break }
                        self.writeIdentifier();
                        if self.cache.is_some() { break }
                    }

                    self.writeSymbol(';');
                    if self.cache.is_some() { return }
                }
                self.closeNonTerminal("varDec");
            },
            _ => self.cache(t),
        }
    }

    /// statements: statement*
    /// statement: (letStatement | ifStatement | whileStatement | doStatement | returnStatement)
    fn compileStatements(&mut self) {
        self.openNonTerminal("statements");
        loop {
            let t = self.token();
            match t.keyword() {
                Some("let") => self.compileLet(),
                Some("if") => self.compileIf(),
                Some("while") => self.compileWhile(),
                Some("do") => self.compileDo(),
                Some("return") => self.compileReturn(),
                _ => { self.cache(t); break },
            }
        }
        self.closeNonTerminal("statements");
    }

    /// 'let' varName ('[' expression ']')? '=' expression ';'
    fn compileLet(&mut self) {
        self.openNonTerminal("letStatement");
        {
            self.writeKeyword("let");
            self.writeIdentifier();
            self.writeSymbol('=');

            if self.cache.is_some() {
                self.writeSymbol('[');
                self.compileExpression();
                self.writeSymbol(']');
                self.writeSymbol('=');
            }

            self.compileExpression();
            self.writeSymbol(';');
        }
        self.closeNonTerminal("letStatement");
    }

    /// 'if' '(' expression ')' '{' statements '}'
    /// ('else' '{' expression '}')?
    fn compileIf(&mut self) {
        self.openNonTerminal("ifStatement");
        {
            self.writeKeyword("if");
            self.writeSymbol('(');
            self.compileExpression();
            self.writeSymbol(')');
            self.writeSymbol('{');
            self.compileStatements();
            self.writeSymbol('}');

            let t = self.token();
            if Some("else") == t.keyword() {
                self.writeKeyword("else");
                self.writeSymbol('{');
                self.compileStatements();
                self.writeSymbol('}');
            } else {
                self.cache(t);
            }
        }
        self.closeNonTerminal("ifStatement");
    }

    /// 'while' '(' expression ')' '{' statements '}'
    fn compileWhile(&mut self) {
        self.openNonTerminal("whileStatement");
        {
            self.writeKeyword("while");
            self.writeSymbol('(');
            self.compileExpression();
            self.writeSymbol(')');
            self.writeSymbol('{');
            self.compileStatements();
            self.writeSymbol('}');
        }
        self.closeNonTerminal("whileStatement");
    }

    /// 'do' subroutineCall ';'
    fn compileDo(&mut self) {
        self.openNonTerminal("doStatement");
        {
            self.writeKeyword("do");
            self.writeSubroutineCall();
            self.writeSymbol(';');
        }
        self.closeNonTerminal("doStatement");
    }

    /// 'return' expression? ';'
    fn compileReturn(&mut self) {
        self.openNonTerminal("returnStatement");
        {
            self.writeKeyword("return");
            self.writeSymbol(';');

            if self.cache.is_some() {
                self.compileExpression();
                self.writeSymbol(';');
            }
        }
        self.closeNonTerminal("returnStatement");
    }

    /// term (op term)*
    fn compileExpression(&mut self) {
        const OPS: [char; 9] = ['+', '-', '*', '/', '&', '|', '<', '>', '='];
        self.openNonTerminal("expression");
        {
            self.compileTerm();

            loop {
                let t = self.token();
                match t.symbol() {
                    Some(&o) if OPS.contains(&o) => {
                        self.cache(t);
                        self.writeSymbol(o);
                        self.compileTerm();
                    },
                    _ => { self.cache(t); break },
                }
            }
        }
        self.closeNonTerminal("expression");
    }

    /// integerConstant | stringConstant | keywordConstant |
    /// varName | varName '[' expression ']' | subroutineCall |
    /// '(' expression ')' | unaryOp term
    fn compileTerm(&mut self) {
        self.openNonTerminal("term");
        {
            let t = self.token();
            match t.symbol() {
                Some(&'(') => {
                    self.cache(t);
                    self.writeSymbol('(');
                    self.compileExpression();
                    self.writeSymbol(')');
                    self.closeNonTerminal("term");
                    return
                },
                Some('-') | Some('~') => {
                    let c: char = *t.symbol().unwrap();  // Copy
                    self.cache(t);
                    self.writeSymbol(c);
                    self.compileTerm();
                    self.closeNonTerminal("term");
                    return
                },
                _ => self.cache(t),
            }

            self.writeIntegerConstant();
            self.writeStringConstant();
            self.writeKeywordConstant();

            self.writeIdentifier();

            let t = self.token();
            match t.symbol() {
                Some('[') => {
                    self.cache(t);
                    self.writeSymbol('[');
                    self.compileExpression();
                    self.writeSymbol(']');
                },
                Some('(') | Some('.') => {
                    self.cache(t);
                    self.writeSubroutineCall();
                },
                _ => self.cache(t),
            }
        }
        self.closeNonTerminal("term");
    }

    /// (expression (',' expression)*)?
    fn compileExpressionList(&mut self) {
        self.openNonTerminal("expressionList");
        {
            let t = self.token();
            if Some(&')') == t.symbol() {
                self.cache(t);
                self.closeNonTerminal("expressionList");
                return
            } else {
                self.cache(t);
            }

            self.compileExpression();

            loop {
                self.writeSymbol(',');
                if self.cache.is_some() { break }
                self.compileExpression();
            }
        }
        self.closeNonTerminal("expressionList");
    }
}
