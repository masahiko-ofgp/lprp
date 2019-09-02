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
pub enum ReaderError {
    SyntaxError,
    NotSymbol,
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ReaderError::SyntaxError => f.write_str("Syntax Error"),
            ReaderError::NotSymbol => f.write_str("Not Symbol"),
        }
    }
}

impl Error for ReaderError {
    fn description(&self) -> &str {
        match *self {
            ReaderError::SyntaxError => "SyntaxError",
            ReaderError::NotSymbol => "Not Symbol",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
    Atom,
    Quote,
    Cons,
    Car,
    Cdr,
}

fn to_symbol<'a>(s: &'a str) -> Result<Symbol, ReaderError> {
    match &s[..] {
        "atom" => Ok(Symbol::Atom),
        "quote" => Ok(Symbol::Quote),
        "cons" => Ok(Symbol::Cons),
        "car" => Ok(Symbol::Car),
        "cdr" => Ok(Symbol::Cdr),
        _ => Err(ReaderError::NotSymbol),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Nil,
    Sym(Box<Symbol>),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Token>),
}

impl Token {
    pub fn gets(&self) -> Option<String> {
        match self {
            Token::Str(s) => Some(s.to_string()),
            _ => None
        }
    }
    pub fn getsym(&self) -> Option<Symbol> {
        match self {
            Token::Sym(sym) => Some(**sym),
            _ => None,
        }
    }
    pub fn getl(&self) -> Option<Vec<Self>> {
        match self {
            Token::List(l) => Some(l.to_vec()),
            _ => None
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
                    match to_symbol(&s) {
                        Ok(sym) => return Token::Sym(Box::new(sym)),
                        Err(_) => {
                            if (tls::strcmp(&vc, "nil"))|(tls::strcmp(&vc, "NIL")) {
                                return Token::Nil;
                            } else {
                                return Token::Str(s.to_string());
                            }
                        }
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

pub fn read<'a>(expr: &'a str) -> Result<Token, ReaderError> {
    let mut chars = expr.chars().peekable();

    let mut token = Token::Nil;

    match chars.peek() {
        Some(c) => {
            if c == &'(' {
                token = read_list(&mut chars);
                Ok(token)
            } else if c == &')' {
                Err(ReaderError::SyntaxError)
            } else {
                token = read_atom(&mut chars);
                Ok(token)
            }
        },
        None => Ok(token),
    }
}
