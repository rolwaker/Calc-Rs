use std::collections::HashMap;
use std::slice::Iter;
use std::iter::Peekable;

use super::number::Number;
use super::token::Token;

fn parse_literal(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    match toks.next() {
        Some(Token::Ident(str)) => {
            match vars {
                Some(map) => {
                    match map.get(str.as_str()) {
                        Some(val) => Ok(val.clone()),
                        None => Err(format!("'{str}' has not been defined"))
                    }
                },
                None => Err("variables cannot be used in this mode".to_string())
            }
        },
        Some(Token::Number(val)) => Ok(val.clone()),
        Some(Token::LeftParen) => {
            let num = match parse_term(vars, toks) {
                Ok(val) => val,
                Err(msg) => return Err(msg)
            };
            match toks.next() {
                Some(Token::RightParen) => Ok(num),
                _ => Err("expected ')'".to_string())
            }
        },
        _ => Err("expected an integer or '('".to_string())
    }
}

fn parse_prefix(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    match toks.peek() {
        Some(Token::Add) => {
            toks.next();
            parse_prefix(vars, toks)
        },
        Some(Token::Subtract) => {
            toks.next();
            match parse_prefix(vars, toks) {
                Ok(mut num) => {
                    num.neg();
                    Ok(num)
                },
                Err(msg) => Err(msg)
            }
        },
        _ => parse_literal(vars, toks)
    }
}

fn parse_power(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    let mut lhs = match parse_prefix(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Power) => {
                toks.next();
                match parse_power(vars, toks) {
                    Ok(rhs) =>
                        match lhs.pow(&rhs) {
                            Ok(()) => {},
                            Err(msg) => return Err(msg)
                        },
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(lhs)
        }
    }
}

fn parse_product(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    let mut lhs = match parse_power(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Multiply) => {
                toks.next();
                match parse_power(vars, toks) {
                    Ok(rhs) => lhs.mul(&rhs),
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Divide) => {
                toks.next();
                match parse_power(vars, toks) {
                    Ok(rhs) => 
                        match lhs.div(&rhs) {
                            Ok(()) => {},
                            Err(msg) => return Err(msg),
                        },
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Modulo) => {
                toks.next();
                match parse_power(vars, toks) {
                    Ok(rhs) => 
                        match lhs.rem(&rhs) {
                            Ok(()) => {},
                            Err(msg) => return Err(msg),
                        },
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(lhs)
        }
    }
}

fn parse_term(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    let mut lhs = match parse_product(vars, toks) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    
    loop {
        match toks.peek() {
            Some(Token::Add) => {
                toks.next();
                match parse_product(vars, toks) {
                    Ok(rhs) => lhs.add(&rhs),
                    Err(msg) => return Err(msg),
                }
            },
            Some(Token::Subtract) => {
                toks.next();
                match parse_product(vars, toks) {
                    Ok(rhs) => lhs.sub(&rhs),
                    Err(msg) => return Err(msg),
                }
            },
            _ => return Ok(lhs)
        }
    }
}

pub fn parse_expr(
        vars: &Option<&mut HashMap<String, Number>>,
        toks: &mut Peekable<Iter<Token>>) -> Result<Number, String> {
    parse_term(vars, toks)
}
