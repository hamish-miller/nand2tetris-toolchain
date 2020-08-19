/// JackTokenizer: Translate input stream into Jack-language tokens

use std::fs;
use std::path::Path;
use std::str::FromStr;

pub struct JackTokenizer {
    source: String,
    cursor: usize,
    pub token: Option<Token>,
}

impl JackTokenizer {
    #![allow(non_snake_case)]  // Contract pre-specified

    pub fn new(path: &Path) -> Self {
        JackTokenizer {
            source: fs::read_to_string(path).unwrap(),
            cursor: 0,
            token: None,
        }
    }

    pub fn hasMoreTokens(&self) -> bool {
        let unparsed = &self.source[self.cursor..];
        !(unparsed == "\n" || unparsed.is_empty())
    }

    pub fn advance(&mut self) {
        let unparsed = &self.source[self.cursor..];

        if let Ok(whitespace) = unparsed.parse::<Whitespace>() {
            self.cursor += whitespace.length;
            self.token = None;
            return;
        }

        if let Ok(comment) = unparsed.parse::<Comment>() {
            self.cursor += comment.length;
            self.token = None;
            return;
        }

        if let Ok(token) = unparsed.parse::<Token>() {
            self.cursor += token.len();
            self.token = Some(token);
            return;
        }

        panic!("Failed to parse: {}", unparsed.lines().next().unwrap());
    }
}

trait Len { fn len(&self) -> usize; }

pub struct ParseError;

struct Whitespace { length: usize }
impl FromStr for Whitespace {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if code.chars().next().unwrap().is_whitespace() {
            Ok(Whitespace { length: code.find(|c: char| !c.is_whitespace()).unwrap() })
        } else {
            Err(Self::Err {})
        }
    }
}

struct Comment { length: usize }
impl FromStr for Comment {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match &code[0..2] {
            "//" => Ok(Comment { length: code.find('\n').unwrap() }),
            "/*" => Ok(Comment { length: code.find("*/").unwrap() + 2 }),
            _ => Err(Self::Err {}),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(Identifier),
    IntConst(IntConst),
    StringConst(StringConst),
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if let Ok(keyword) = code.parse::<Keyword>() {
            return Ok(Token::Keyword(keyword))
        }

        if let Ok(symbol) = code.parse::<Symbol>() {
            return Ok(Token::Symbol(symbol))
        }

        if let Ok(identifier) = code.parse::<Identifier>() {
            return Ok(Token::Identifier(identifier))
        }

        if let Ok(identifier) = code.parse::<IntConst>() {
            return Ok(Token::IntConst(identifier))
        }

        if let Ok(identifier) = code.parse::<StringConst>() {
            return Ok(Token::StringConst(identifier))
        }

        Err(Self::Err {})
    }
}

impl Len for Token {
    fn len(&self) -> usize {
        use Token::*;
        match self {
            Keyword(x) => x.len(),
            Symbol(x) => x.len(),
            Identifier(x) => x.len(),
            IntConst(x) => x.len(),
            StringConst(x) => x.len(),
        }
    }
}

#[derive(Debug)]
struct Keyword(String);
const KEYWORDS: [&'static str; 21] = [
    "class", "constructor", "function",
    "method", "field", "static", "var",
    "int", "char", "boolean", "void", "true",
    "false", "null", "this", "let", "do",
    "if", "else", "while", "return",
];

impl FromStr for Keyword {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let word = code.split_whitespace().next().unwrap();

        if KEYWORDS.contains(&word) {
            Ok(Keyword(word.to_string()))
        } else {
            Err(Self::Err {})
        }

    }
}

impl Len for Keyword { fn len(&self) -> usize { self.0.len() } }


#[derive(Debug)]
struct Symbol(char);
const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.',
    ',', ';', '+', '-', '*', '/', '&',
    '|', '<', '>', '=', '-',
];

impl FromStr for Symbol {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let c = code.chars().next().unwrap();

        if SYMBOLS.contains(&c) {
            Ok(Symbol(c))
        } else {
            Err(Self::Err {})
        }

    }
}

impl Len for Symbol { fn len(&self) -> usize { 1 } }

#[derive(Debug)]
struct Identifier(String);

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let non_ascii = |c: char| { !(c.is_ascii_alphanumeric() || c == '_') };
        let c = code.chars().next().unwrap();

        if c.is_ascii_alphabetic() || c == '_' {
            let until = code.find(non_ascii).unwrap();
            Ok(Identifier(code[..until].to_string()))
        } else {
            Err(Self::Err {})
        }

    }
}

impl Len for Identifier { fn len(&self) -> usize { self.inner.len() } }

#[derive(Debug)]
struct IntConst(u16);

impl FromStr for IntConst {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let non_digit = |c: char| { !c.is_ascii_digit() };
        let c = code.chars().next().unwrap();

        if c.is_ascii_digit() {
            let until = code.find(non_digit).unwrap();
            Ok(IntConst(code[..until].parse::<u16>().unwrap()))
        } else {
            Err(Self::Err {})
        }
    }
}

impl Len for IntConst { fn len(&self) -> usize { self.0.to_string().chars().count() } }

#[derive(Debug)]
struct StringConst(String);

impl FromStr for StringConst {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if code.chars().next() == Some('"') {
            let until = code[1..].find('"').unwrap() + 1;
            Ok(StringConst(code[1..until].to_string()))
        } else {
            Err(Self::Err {})
        }
    }
}

impl Len for StringConst { fn len(&self) -> usize { self.0.len() + 2 } }
