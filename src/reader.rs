// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use std::iter::Peekable;
use std::fmt;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum ReaderError {
    SyntaxError,
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ReaderError::SyntaxError => f.write_str("Syntax Error"),
        }
    }
}

impl Error for ReaderError {
    fn description(&self) -> &str {
        match *self {
            ReaderError::SyntaxError => "ERROR: SyntaxError",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ERROR,
    Nil,
    Str(String),
    List(Vec<Token>),
}

#[allow(dead_code)]
impl Token {
    pub fn gets(self) -> Option<String> {
        match self {
            Token::Str(s) => Some(s.to_string()),
            _ => None
        }
    }
    pub fn getl(self) -> Option<Vec<Self>> {
        match self {
            Token::List(v) => Some(v.to_vec()),
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
                if is_tk_str(&c) {
                    let s = read_str(&mut chars);
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

fn read_str<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    let mut s = String::new();

    loop {
        match chars.peek() {
            Some(c) if is_tk_str(&c) => {
                s.push(*c);
            }
            _ => return Token::Str(s.to_string()),
        }
        chars.next();
    }
}

// Helper function
fn is_tk_str(ch: &char) -> bool {
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
                token = read_str(&mut chars);
                Ok(token)
            }
        },
        None => Ok(token),
    }
}
