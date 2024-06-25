use std::str::Chars;
use std::iter::Peekable;

use super::number::Number;
use super::token::Token;

fn lex_identifier(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut var = String::new();
    loop {
        match buffer.peek() {
            Some('a'..='z' | 'A'..='Z' | '0'..='9') => var.push(buffer.next().unwrap()),
            _ => return Ok(Token::Ident(var))
        }
    }
}

fn lex_operator(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    match buffer.next() {
        Some('(') => Ok(Token::LeftParen),
        Some(')') => Ok(Token::RightParen),
        Some('+') => Ok(Token::Add),
        Some('-') => Ok(Token::Subtract),
        Some('*') => Ok(Token::Multiply),
        Some('/') => Ok(Token::Divide),
        Some('%') => Ok(Token::Modulo),
        Some('^') => Ok(Token::Power),
        Some('=') => Ok(Token::Assign),
        _ => Err("impossible".to_string())
    }
}

fn lex_numeral(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    let mut digits = String::new();
    
    loop {
        match buffer.peek() {
            Some('0'..='9') => {
                digits.push(buffer.next().unwrap());
            },
            _ => break
        }
    }
    
    Ok(Token::Number(Number::from(&digits).unwrap()))
}


fn lex_token(buffer: &mut Peekable<Chars>) -> Result<Token, String> {
    match buffer.peek() {
        Some('a'..='z' | 'A'..='Z') => lex_identifier(buffer),
        Some('(' | ')' | '+' | '-' | '*' | '/' | '%' | '^' | '=') => lex_operator(buffer),
        Some('0'..='9') => lex_numeral(buffer),
        Some(c) => Err(format!("unknown character: '{c}'")),
        None => Err("unexpected end-of-file".to_string())
    }
}


fn lex_whitespace(buffer: &mut Peekable<Chars>) -> bool {
    loop {
        match buffer.peek() {
            Some(' ' | '\t' | '\n') => {
                buffer.next();
            },
            Some(_) => return true,
            None => return false
        }
    }
}


pub fn lex_string(string: &str) -> Result<Vec<Token>, String> {
    let mut toks = Vec::new();
    let mut chars = string.chars().peekable();
    
    while lex_whitespace(&mut chars) {
        match lex_token(&mut chars) {
            Ok(tok) => toks.push(tok),
            Err(msg) => return Err(msg)
        }
    }
    
    Ok(toks)
}
