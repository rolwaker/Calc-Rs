use std::collections::HashMap;
use std::slice::Iter;
use std::iter::Peekable;

use super::number::Number;
use super::token::Token;
use super::arith::parse_expr;

fn has_assign(it: &mut Peekable<Iter<'_, Token>>) -> Option<String> {
    match it.peek() {
        Some(Token::Ident(key)) => {
            it.next();
            match it.peek() {
                Some(Token::Assign) => {
                    it.next();
                    Some(key.clone())
                },
                _ => None
            }
        },
        _ => None
    }
}

pub fn parse_line(vars: &mut HashMap<String, Number>, toks: &Vec<Token>) -> Result<Number, String> {
    let mut it = toks.iter().peekable();
    match has_assign(&mut it) {
        Some(key) => {
            let result = parse_expr(&Some(vars), &mut it);
            if let Ok(val) = &result {
                vars.insert(key, val.clone());
            }
            result
        },
        None => {
            // use new iter since the old one may have advanced past the first token.
            parse_expr(&Some(vars), &mut toks.iter().peekable())
        }
    }
}
