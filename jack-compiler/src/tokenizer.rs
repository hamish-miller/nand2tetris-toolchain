/// JackTokenizer: Translate input stream into Jack-language tokens

use std::fs;
use std::mem;
use std::path::Path;
use std::str::FromStr;

pub struct JackTokenizer {
    source: String,
    cursor: usize,
    token: Option<Token>,
}

impl JackTokenizer {
    #![allow(non_snake_case)]  // Contract pre-specified

    pub fn new(source: &str) -> Self {
        JackTokenizer {
            source: String::from(source),
            cursor: 0,
            token: None,
        }
    }

    pub fn open(path: &Path) -> Self {
        JackTokenizer {
            source: fs::read_to_string(path).unwrap(),
            cursor: 0,
            token: None,
        }
    }

    fn hasMoreTokens(&self) -> bool {
        let unparsed = &self.source[self.cursor..];
        !(unparsed == "\n" || unparsed.is_empty())
    }

    fn advance(&mut self) {
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

// Amazingly this works
pub trait TokenStream: Iterator<Item=Token> {}
impl<T: Iterator<Item=Token>> TokenStream for T {}

impl Iterator for JackTokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.hasMoreTokens() {
            self.advance();

            if self.token.is_some() {
                return mem::replace(&mut self.token, None)
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ParseError;

struct Whitespace { length: usize }
impl FromStr for Whitespace {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match code.find(|c: char| !c.is_whitespace()) {
            Some(0) => Err(Self::Err {}),
            Some(l) => Ok(Whitespace { length: l }),
            None => Ok(Whitespace { length: code.len() }),
        }
    }
}

struct Comment { length: usize }
impl FromStr for Comment {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let l = match code.get(0..2) {
            Some("//") => code.find('\n').unwrap_or_else(|| code.len()),
            Some("/*") => code.find("*/").expect("'*/' not found.") + 2,
            _ => return Err(Self::Err {}),
        };

        Ok(Comment { length: l })
    }
}

#[derive(Debug, PartialEq)]
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

impl Token {
    pub fn keyword(&self) -> Option<&str> {
        match self {
            Token::Keyword(k) => Some(&k.0),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<&char> {
        match self {
            Token::Symbol(i) => Some(&i.0),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(i) => Some(&i.0),
            _ => None,
        }
    }

    fn len(&self) -> usize {
        use Token::*;
        match self {
            Keyword(k) => k.0.len(),
            Symbol(_) => 1,
            Identifier(i) => i.0.len(),
            IntConst(i) => i.0.to_string().chars().count(),
            StringConst(s) => s.0.len() + 2,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Keyword(pub String);
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


#[derive(Debug, PartialEq)]
pub struct Symbol(pub char);
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

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

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


#[derive(Debug, PartialEq)]
pub struct IntConst(pub u16);

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


#[derive(Debug, PartialEq)]
pub struct StringConst(pub String);

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
