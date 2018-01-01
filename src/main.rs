#[macro_use]
extern crate lazy_static;

extern crate regex;

use regex::bytes::Regex;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Token {
    Number { value: String, line: i32, column: usize },
    Symbol { value: String, line: i32, column: usize },
    Whitespace { value: String, line: i32, column: usize },
}

fn lex_number(text: &mut String) -> Option<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^-?\d+(\.\d)?").unwrap();
    }
    if let Some(mat) = RE.find(text.clone().as_bytes()) {
        let sub: String = text.drain(..mat.end()).collect();
        Some(Token::Number { value: sub, line: 1, column: 1 })
    } else {
        None
    }
}

fn lex_symbol(text: &mut String) -> Option<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z_]+[\w-]*[!?_]?").unwrap();
    }
    if let Some(mat) = RE.find(text.clone().as_bytes()) {
        let sub: String = text.drain(..mat.end()).collect();
        Some(Token::Symbol { value: sub, line: 1, column: 1 })
    } else {
        None
    }
}

fn lex_whitespace(text: &mut String) -> Option<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s+").unwrap();
    }
    if let Some(mat) = RE.find(text.clone().as_bytes()) {
        let sub: String = text.drain(..mat.end()).collect();
        Some(Token::Whitespace { value: sub, line: 1, column: 1 })
    } else {
        None
    }
}

fn lex(mut text: &mut String) -> Vec<Token> {
    let mut tokens = Vec::new();
    while !text.is_empty() {
        let mut found = false;
        for lexer in [lex_number, lex_symbol, lex_whitespace].iter() {
            if let Some(token) = lexer(&mut text) {
                tokens.push(token);
                found = true;
                break;
            }
        }
        if !found {
            text.clear();
            break;
        }
    }
    tokens
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("amigo 0.1\n> ");
    stdout.flush().unwrap();

    let mut input = String::new();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
        let tokens = lex(&mut input);
        print!("{:?}\n> ", tokens);
        stdout.flush().unwrap();
    }
}
