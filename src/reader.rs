// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Nil,
    Num(u32),
    Str(String),
    List(Vec<Token>),
}

fn read_list<I>(mut chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    chars.next();

    let mut v: Vec<Token> = vec![];

    loop {
        match chars.peek() {
            Some(c) => {
                if c.is_digit(10) {
                    let n = read_num(&mut chars);
                    v.push(n);
                    chars.next();
                } else if c.is_alphabetic() {
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
            Some(c) if c.is_alphabetic() => {
                s.push(*c);
            }
            _ => return Token::Str(s.to_string()),
        }
        chars.next();
    }
}

fn read_num<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match chars.peek() {
            Some(c) if c.is_digit(10) => {
                n = n * 10 + c.to_digit(10).unwrap();
            }
            _ => return Token::Num(n),
        }
        chars.next();
    }
}

pub fn read<'a>(expr: &'a str) -> Token {
    let mut chars = expr.chars().peekable();

    let mut token = Token::Nil;

    loop {
        match chars.peek() {
            Some(c) => {
                if c == &'(' {
                    token = read_list(&mut chars);
                } else if c.is_digit(10) {
                    token = read_num(&mut chars);
                } else if c.is_alphabetic() {
                    token = read_str(&mut chars);
                } else {
                    return token
                }
            },
            None => return token,
        }
    }
}
