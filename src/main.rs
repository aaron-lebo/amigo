#[macro_use]
extern crate lazy_static;

extern crate regex;

use regex::bytes::Regex;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Token {
    Number { val: String, line: i32, col: u32, len: usize },
    Symbol { val: String, line: i32, col: u32, len: usize },
    LeftParen { val: String, line: i32, col: u32, len: usize },
    RightParen { val: String, line: i32, col: u32, len: usize },
    Whitespace { val: String, line: i32, col: u32, len: usize },
}

struct Lexer {
    buf: String,
    pos: u32,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer { buf: String::new(), pos: 1 }
    }

    fn take(&mut self, end: usize) -> String {
        self.pos += end as u32;
        self.buf.drain(..end).collect()
    }

    fn lex_number(&mut self) -> Option<Token> {
        lazy_static! { static ref RE: Regex = Regex::new(r"^-?\d+(\.\d)?").unwrap(); }
        if let Some(mat) = RE.find(self.buf.clone().as_bytes()) {
            let (col, len) = (self.pos, mat.end());
            Some(Token::Number { val: self.take(len), line: 1, col: col, len: len })
        } else {
            None
        }
    }

    fn lex_symbol(&mut self) -> Option<Token> {
        lazy_static! { static ref RE: Regex = Regex::new(r"^[a-zA-Z_]+[\w-]*[!?_]?").unwrap(); }
        if let Some(mat) = RE.find(self.buf.clone().as_bytes()) {
            let (col, len) = (self.pos, mat.end());
            Some(Token::Symbol { val: self.take(len), line: 1, col: col, len: len })
        } else {
            None
        }
    }

    fn lex_left_paren(&mut self) -> Option<Token> {
        lazy_static! { static ref RE: Regex = Regex::new(r"^\(").unwrap(); }
        if let Some(mat) = RE.find(self.buf.clone().as_bytes()) {
            let (col, len) = (self.pos, mat.end());
            Some(Token::LeftParen { val: self.take(len), line: 1, col: col, len: len })
        } else {
            None
        }
    }

    fn lex_right_paren(&mut self) -> Option<Token> {
        lazy_static! { static ref RE: Regex = Regex::new(r"^\)").unwrap(); }
        if let Some(mat) = RE.find(self.buf.clone().as_bytes()) {
            let (col, len) = (self.pos, mat.end());
            Some(Token::RightParen { val: self.take(len), line: 1, col: col, len: len })
        } else {
            None
        }
    }

    fn lex_whitespace(&mut self) -> Option<Token> {
        lazy_static! { static ref RE: Regex = Regex::new(r"^\s+").unwrap(); }
        if let Some(mat) = RE.find(self.buf.clone().as_bytes()) {
            let (col, len) = (self.pos, mat.end());
            Some(Token::Whitespace { val: self.take(len), line: 1, col: col, len: len })
        } else {
            None
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut token: Option<Token> = None;
        for lex in &[Self::lex_symbol, Self::lex_number, Self::lex_left_paren, Self:: lex_right_paren, Self::lex_whitespace] {
            token = lex(self);
            if token.is_some() {
                break;
            }
        }
        token
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("amigo 0.3\n> ");
    stdout.flush().unwrap();

    let mut lexer = Lexer::new();
    for line in stdin.lock().lines() {
        lexer.buf.push_str(&line.unwrap());
        lexer.pos = 1;
        for token in &mut lexer {
            println!("{:?}", token);
        }
        print!("> ");
        stdout.flush().unwrap();
    }
}
