// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use std::iter::Peekable;
use std::fmt;
use std::error::Error;
use onigiri::tools as tls;
use onigiri::validator as vld;


#[derive(Debug, PartialEq, Clone)]
pub enum LprpError {
    SyntaxError,
}

impl fmt::Display for LprpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            LprpError::SyntaxError => f.write_str("Syntax Error"),
        }
    }
}

impl Error for LprpError {
    fn description(&self) -> &str {
        match *self {
            LprpError::SyntaxError => "SyntaxError",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Nil,
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Token>),
}


impl Token {
    pub fn geti(&self) -> Result<i64, LprpError> {
        match self {
            Token::Int(i) => Ok(*i),
            _ => Err(LprpError::SyntaxError)
        }
    }
    pub fn getf(&self) -> Result<f64, LprpError> {
        match self {
            Token::Float(f) => Ok(*f),
            _ => Err(LprpError::SyntaxError)
        }
    }
    pub fn gets(&self) -> Result<String, LprpError> {
        match self {
            Token::Str(s) => Ok(s.to_string()),
            _ => Err(LprpError::SyntaxError)
        }
    }
    pub fn getl(&self) -> Result<Vec<Self>, LprpError> {
        match self {
            Token::List(l) => Ok(l.to_vec()),
            _ => Err(LprpError::SyntaxError)
        }
    }
    pub fn car(&self) -> Result<Self, LprpError> {
        match self {
            Token::List(l) => {
                match l.first() {
                    Some(tk) => Ok(tk.clone()),
                    None => Ok(Token::Nil),
                }
            },
            _ => Err(LprpError::SyntaxError),
        }
    }
    pub fn cdr(&self) -> Result<Self, LprpError> {
        match self {
            Token::List(l) => {
                match l.split_first() {
                    Some((_, tl)) => {
                        match tl.len() {
                            0 => Ok(Token::List(vec![Token::Nil])),
                            1 => Ok(Token::List(vec![tl[0].clone()])),
                            _ => Ok(Token::List(tl.to_vec())),
                        }
                    },
                    None => Ok(Token::Nil),
                }
            },
            _ => Err(LprpError::SyntaxError),
        }
    }
}

fn read_list<I>(mut chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    chars.next();

    let mut v: Vec<Token> = vec![];

    loop {
        match chars.peek() {
            Some(c) => {
                if is_tk_atom(&c) {
                    let s = read_atom(&mut chars);
                    v.push(s);
                    chars.next();
                } else if c == &'(' {
                    let l = read_list(&mut chars);
                    v.push(l);
                    chars.next();
                } else {
                    return Token::List(v.to_vec());
                }
            },
            None => return Token::List(v.to_vec()),
        }
    }
}

fn read_atom<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    let mut vc: Vec<char> = vec![];

    loop {
        match chars.peek() {
            Some(c) if is_tk_atom(&c) => {
                vc.push(*c);
            }
            _ => {
                if vld::is_integer(&vc) {
                    return Token::Int(tls::cast::<i64>(&vc).unwrap());
                } else if vld::is_float(&vc) {
                    return Token::Float(tls::cast::<f64>(&vc).unwrap());
                } else {
                    let s = tls::chars_to_string(&vc);
                    if (tls::strcmp(&vc, "nil"))|(tls::strcmp(&vc, "NIL")) {
                        return Token::Nil;
                    } else {
                        return Token::Str(s.to_string());
                    }
                }
            }
        }
        chars.next();
    }
}

// Check whether the character can be used in Token
fn is_tk_atom(ch: &char) -> bool {
    match ch {
        'a' ..= 'z'|'A' ..= 'Z' => true,
        '0' ..= '9' => true,
        '+'|'-'|'*'|'/' => true,
        '\"'|'\''|':'|'.' => true,
        _ => false,
    }
}

pub fn read<'a>(expr: &'a str) -> Result<Token, LprpError> {
    let mut chars = expr.chars().peekable();

    let mut token = Token::Nil;

    match chars.peek() {
        Some(c) => {
            if c == &'(' {
                token = read_list(&mut chars);
                Ok(token)
            } else if c == &')' {
                Err(LprpError::SyntaxError)
            } else {
                token = read_atom(&mut chars);
                Ok(token)
            }
        },
        None => Ok(token),
    }
}
