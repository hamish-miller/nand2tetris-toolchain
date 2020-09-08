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
    Keyword(String),
    Symbol(char),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        if let Ok(keyword) = parse_keyword(&code) {
            return Ok(Token::Keyword(keyword))
        }

        if let Ok(symbol) = parse_symbol(&code) {
            return Ok(Token::Symbol(symbol))
        }

        if let Ok(identifier) = parse_identifier(&code) {
            return Ok(Token::Identifier(identifier))
        }

        if let Ok(int) = parse_int_const(&code) {
            return Ok(Token::IntConst(int))
        }

        if let Ok(string) = parse_string_const(&code) {
            return Ok(Token::StringConst(string))
        }

        Err(Self::Err {})
    }
}

impl Token {
    pub fn keyword(&self) -> Option<&str> {
        match self {
            Token::Keyword(k) => Some(&k),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<&char> {
        match self {
            Token::Symbol(i) => Some(&i),
            _ => None,
        }
    }

    pub fn identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(i) => Some(&i),
            _ => None,
        }
    }

    pub fn integer_constant(&self) -> Option<&str> {
        match self {
            Token::IntConst(i) => Some(&i),
            _ => None,
        }
    }

    pub fn string_constant(&self) -> Option<&str> {
        match self {
            Token::StringConst(i) => Some(&i),
            _ => None,
        }
    }

    fn len(&self) -> usize {
        use Token::*;
        match self {
            Keyword(k) => k.len(),
            Symbol(_) => 1,
            Identifier(i) => i.len(),
            IntConst(i) => i.chars().count(),
            StringConst(s) => s.len() + 2,
        }
    }
}


const KEYWORDS: [&'static str; 21] = [
    "class", "constructor", "function",
    "method", "field", "static", "var",
    "int", "char", "boolean", "void", "true",
    "false", "null", "this", "let", "do",
    "if", "else", "while", "return",
];

fn parse_keyword(code: &str) -> Result<String, ParseError> {
    if let Some(i) = code.find(|c: char| !c.is_ascii_alphabetic()) {
        let word = &code[..i];

        if KEYWORDS.contains(&word) {
            return Ok(word.to_string())
        }
    }

    Err(ParseError)
}


const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.',
    ',', ';', '+', '-', '*', '/', '&',
    '|', '<', '>', '=', '~',
];

fn parse_symbol(code: &str) -> Result<char, ParseError> {
    let c = code.chars().next().unwrap();

    if SYMBOLS.contains(&c) {
        Ok(c)
    } else {
        Err(ParseError)
    }
}


fn parse_identifier(code: &str) -> Result<String, ParseError> {
    let non_ascii = |c: char| { !(c.is_ascii_alphanumeric() || c == '_') };
    let c = code.chars().next().unwrap();

    if c.is_ascii_alphabetic() || c == '_' {
        let until = code.find(non_ascii).unwrap();
        Ok(code[..until].to_string())
    } else {
        Err(ParseError)
    }
}


fn parse_int_const(code: &str) -> Result<String, ParseError> {
    let non_digit = |c: char| { !c.is_ascii_digit() };
    let c = code.chars().next().unwrap();

    if c.is_ascii_digit() {
        let until = code.find(non_digit).unwrap();
        Ok(code[..until].to_string())
    } else {
        Err(ParseError)
    }
}


fn parse_string_const(code: &str) -> Result<String, ParseError> {
    if code.chars().next() == Some('"') {
        let until = code[1..].find('"').unwrap() + 1;
        Ok(code[1..until].to_string())
    } else {
        Err(ParseError)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword() {
        let mut t = JackTokenizer::new("class Foo {}");
        t.advance();

        assert_eq!(t.token, Some(Token::Keyword(String::from("class"))));
    }

    #[test]
    fn test_parse_symbol() {
        let mut t = JackTokenizer::new("class Foo {}");
        for _ in 0..5 { t.advance(); }

        assert_eq!(t.token, Some(Token::Symbol('{')));
    }

    #[test]
    fn test_parse_identifier() {
        let mut t = JackTokenizer::new("class Foo {}");
        for _ in 0..3 { t.advance(); }

        assert_eq!(t.token, Some(Token::Identifier(String::from("Foo"))));
    }

    #[test]
    fn test_parse_int_const() {
        let mut t = JackTokenizer::new("let meaning = 42;");
        for _ in 0..7 { t.advance(); }

        assert_eq!(t.token, Some(Token::IntConst(String::from("42"))));
    }
    #[test]
    fn test_parse_string_const() {
        let mut t = JackTokenizer::new("let hello = \"world\";");
        for _ in 0..7 { t.advance(); }

        assert_eq!(t.token, Some(Token::StringConst(String::from("world"))));
    }
}
