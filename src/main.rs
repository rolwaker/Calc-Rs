use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, Read, Write, stdin, stdout};
use std::path::Path;

mod arith;
mod bignum;
mod lex;
mod number;
mod state;
mod token;

use number::Number;
use token::Token;

fn eval_print<Parse>(string: &str, mut parse: Parse) -> Result<(), i32>
        where Parse: FnMut(&Vec<Token>) -> Result<Number, String> {
    match lex::lex_string(string) {
        Ok(toks) => {
            match parse(&toks) {
                Ok(n) => {
                    println!("{}", n.to_string());
                    Ok(())
                },
                Err(msg) => {
                    println!("parse error: {msg}!");
                    Err(1)
                }
            }
        },
        Err(msg) => {
            println!("lex error: {msg}!");
            Err(1)
        }
    }
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    let mut buffer;
    
    if args.len() < 2 {
        let stdin = stdin();
        let stdout = stdout();
        let mut vars = HashMap::new();
        
        loop {
            let mut out = stdout.lock();
            out.write(b"#").unwrap();
            out.flush().unwrap();
            buffer = stdin.lock().lines().next().unwrap().unwrap();
            
            match buffer.as_str() {
                "exit" | "quit" | "!" => return Ok(()),
                _ => _ = eval_print(&buffer, |toks| state::parse_line(&mut vars, toks))
            }
        }
    } else if args.len() == 2 && !args[1].chars().nth(0).unwrap_or(' ').is_digit(10) {
        buffer = String::new();
        let path = Path::new(args.get(1).unwrap());
        let disp = path.display();
        
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_string(&mut buffer) {
                    Ok(_) => {
                        let mut ec = 0;
                        let mut vars = HashMap::new();
                        
                        for line in buffer.lines() {
                            println!("#{}", line);
                            
                            match eval_print(line, |toks| state::parse_line(&mut vars, toks)) {
                                Err(c) => {
                                    ec = c;
                                },
                                Ok(_) => {}
                            }
                        }
                        
                        if ec != 0 {
                            Err(ec)
                        } else {
                            Ok(())
                        }
                    },
                    Err(msg) => {
                        println!("could not read file '{disp}', error: {msg}!");
                        Err(-1)
                    }
                }
            },
            Err(msg) => {
                println!("could not open file '{disp}', error: {msg}!");
                Err(-1)
            }
        }
    } else {
        buffer = args[1..].join(" ");
        eval_print(&buffer, |toks| arith::parse_expr(&mut None, &mut toks.iter().peekable()))
    }
}
