// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use std::fmt;
use std::error::Error;
use std::iter::Peekable;
use onigiri::tools as tls;
use onigiri::validator as vld;


#[derive(Debug, PartialEq, Clone)]
pub enum LprpError {
    SyntaxError,
    ReadError,
    ReadNumError,
}

impl fmt::Display for LprpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Self::SyntaxError => f.write_str("Syntax Error"),
            Self::ReadError => f.write_str("Read Error"),
            Self::ReadNumError => f.write_str("Read Num Error"),
        }
    }
}

impl Error for LprpError {
    fn description(&self) -> &str {
        match *self {
            Self::SyntaxError => "SyntaxError",
            Self::ReadError => "ReadError",
            Self::ReadNumError => "ReadNumError",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    T,
    Nil,
    Int(i64),
    Float(f64),
    Symbol(String),
    Keyword(String),
    Special(String),
    Quote(Box<Token>),
    Str(String),
    List(Vec<Token>),
}

// ***** Int, Float *****
fn is_lprp_num(ch: &char) -> bool {
    (ch.is_ascii_digit())||(ch == &'-')||(ch == &'.')
}

fn read_num<I>(chars: &mut Peekable<I>) -> Result<Token, LprpError>
    where I: Iterator<Item=char>
{
    let mut num: Vec<char> = vec![];

    loop {
        match chars.peek() {
            Some(c) if is_lprp_num(&c) => {
                num.push(*c);
            }
            _ => {
                if vld::is_integer(&num) {
                    return Ok(Token::Int(tls::cast::<i64>(&num).unwrap()));
                } else if vld::is_float(&num) {
                    return Ok(Token::Float(tls::cast::<f64>(&num).unwrap()));
                } else {
                    return Err(LprpError::ReadNumError);
                }
            }
        }
        chars.next();
    }
}

#[test]
fn test_read_num() {
    let mut i = "123".chars().peekable();
    let mut f = "-0.12".chars().peekable();
    assert_eq!(read_num(&mut i), Ok(Token::Int(123)));
    assert_eq!(read_num(&mut f), Ok(Token::Float(-0.12)));
}

// ***** Symbol *****
fn is_lprp_symbol(ch: &char) -> bool {
    (ch.is_ascii_alphabetic())||(ch == &'-')
}

fn read_symbol<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    let mut sym = String::new();

    loop {
        match chars.peek() {
            Some(c) if is_lprp_symbol(&c) => {
                sym.push(*c);
            }
            _ => {
                if (&sym[..] == "nil")||(&sym[..] == "NIL") {
                    return Token::Nil;
                } else if &sym[..] == "t" {
                    return Token::T;
                } else {
                    return Token::Symbol(sym.to_string());
                }
            }
        }
        chars.next();
    }
}

#[test]
fn test_read_symbol() {
    let mut sym = "with-open".chars().peekable();
    let mut error_sym = "with_open".chars().peekable();
    let mut nil = "nil".chars().peekable();
    let mut t = "t".chars().peekable();
    assert_eq!(read_symbol(&mut sym), Token::Symbol("with-open".to_string()));
    assert_eq!(read_symbol(&mut error_sym), Token::Symbol("with".to_string()));
    assert_eq!(read_symbol(&mut nil), Token::Nil);
    assert_eq!(read_symbol(&mut t), Token::T);
}

// ***** Keyword *****
fn read_keyword<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    chars.next();
    
    let mut k = String::new();

    loop {
        match chars.peek() {
            Some(c) if is_lprp_symbol(&c) => {
                k.push(*c);
            }
            _ => return Token::Keyword(k.to_string()),
        }
        chars.next();
    }
}

#[test]
fn test_read_keyword() {
    let mut key = ":my-key".chars().peekable();
    assert_eq!(read_keyword(&mut key), Token::Keyword("my-key".to_string()));
}

// ***** Special *****
fn is_lprp_special(ch: &char) -> bool {
    (is_lprp_symbol(&ch))||(ch == &'*')
}

fn read_special<I>(chars: &mut Peekable<I>) -> Result<Token, LprpError>
    where I: Iterator<Item=char>
{
    let mut sp: Vec<char> = vec![];;
    
    loop {
        match chars.peek() {
            Some(c) if is_lprp_special(&c) => {
                sp.push(*c);
            }
            _ => {
                let cnt = &sp.iter()
                    .filter(|&c| c == &'*')
                    .count();
                if cnt != &2_usize {
                    return Err(LprpError::SyntaxError);
                } else {
                    let sp2 = tls::chars_to_string(&sp);
                    let sp3 = &sp2.trim_matches('*');
                    return Ok(Token::Special(sp3.to_string()));
                }
            }
        }
        chars.next();
    }
}

#[test]
fn test_read_special() {
    let mut sp = "*special*".chars().peekable();
    let mut e = "*special".chars().peekable();
    let mut e2 = "***".chars().peekable();
    assert_eq!(
        read_special(&mut sp),
        Ok(Token::Special("special".to_string()))
        );
    assert_eq!(read_special(&mut e), Err(LprpError::SyntaxError));
    assert_eq!(read_special(&mut e2), Err(LprpError::SyntaxError));
}

// ***** Str *****
fn read_string<I>(chars: &mut Peekable<I>) -> Token
    where I: Iterator<Item=char>
{
    chars.next();
    
    let mut s = String::new();

    loop {
        match chars.peek() {
            Some(c) if c != &'\"' => {
                s.push(*c);
            }
            _ => {
                chars.next();
                return Token::Str(s.to_string());
            }
        }
        chars.next();
    }
}

#[test]
fn test_read_string() {
    let mut s = "\"(Oops!)\"".chars().peekable();
    assert_eq!(read_string(&mut s), Token::Str("(Oops!)".to_string()));
}

// ***** List *****
fn read_list<I>(mut chars: &mut Peekable<I>) -> Result<Token, LprpError>
    where I: Iterator<Item=char>
{
    chars.next();

    let mut v: Vec<Token> = vec![];

    loop {
        match chars.peek() {
            Some(c) => {
                match *c {
                    '(' => {
                        match read_list(&mut chars) {
                            Ok(l) => {
                                v.push(l);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    ')' => {
                        chars.next();
                        return Ok(Token::List(v.to_vec()));
                    },
                    ' ' => {
                        chars.next();
                    },
                    '0' ..= '9'|'-' => {
                        match read_num(&mut chars) {
                            Ok(n) => {
                                v.push(n);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    'a' ..= 'z'|'A' ..= 'Z' => {
                        v.push(read_symbol(&mut chars));
                    },
                    '*' => {
                        match read_special(&mut chars) {
                            Ok(sp) => {
                                v.push(sp);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    ':' => {
                        v.push(read_keyword(&mut chars));
                    },
                    '\"' => {
                        v.push(read_string(&mut chars));
                    },
                    '\'' => {
                        match read_quote(&mut chars) {
                            Ok(q) => {
                                v.push(q);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    _ => {
                        return Err(LprpError::SyntaxError);
                    }
                }
            },
            _ => return Ok(Token::List(v.to_vec())),
        }
    }
}

#[test]
fn test_read_list() {
    let mut l = "(1 (2 3))".chars().peekable();
    assert_eq!(
        read_list(&mut l),
        Ok(Token::List(vec![
                    Token::Int(1),
                    Token::List(vec![
                                Token::Int(2),
                                Token::Int(3),
                    ])
        ]))
    );
}

// ***** Quote *****
fn read_quote<I>(mut chars: &mut Peekable<I>) -> Result<Token, LprpError>
    where I: Iterator<Item=char>
{
    chars.next();

    match chars.peek() {
        Some(c) => {
            match *c {
                '(' => {
                    match read_list(&mut chars) {
                        Ok(l) => Ok(Token::Quote(Box::new(l))),
                        Err(e) => Err(e)
                    }
                },
                '0' ..= '9'|'-' => {
                    match read_num(&mut chars) {
                        Ok(n) => Ok(Token::Quote(Box::new(n))),
                        Err(e) => Err(e),
                    }
                },
                'a' ..= 'z'|'A' ..= 'Z' => Ok(Token::Quote(Box::new(read_symbol(&mut chars)))),
                ':' => Ok(Token::Quote(Box::new(read_keyword(&mut chars)))),
                '*' => {
                    match read_special(&mut chars) {
                        Ok(sp) => Ok(Token::Quote(Box::new(sp))),
                        Err(e) => Err(e),
                    }
                },
                '\"' => Ok(Token::Quote(Box::new(read_string(&mut chars)))),
                '\'' => {
                    match read_quote(&mut chars) {
                        Ok(q) => Ok(Token::Quote(Box::new(q))),
                        Err(e) => Err(e),
                    }
                },
                _ => Err(LprpError::SyntaxError)
            }
        },
        _ => Err(LprpError::SyntaxError)
    }
}

#[test]
fn test_read_quote() {
    let mut q = "'(1 2 3)".chars().peekable();
    assert_eq!(
        read_quote(&mut q),
        Ok(Token::Quote(
                Box::new(
                    Token::List(vec![
                                Token::Int(1),
                                Token::Int(2),
                                Token::Int(3),
                    ]))
                )
            )
        );
}

fn read_expr<I>(mut chars: &mut Peekable<I>) -> Result<Token, LprpError>
    where I: Iterator<Item=char>
{
    let mut v: Vec<Token> = vec![];

    loop {
        match chars.peek() {
            Some(c) => {
                match *c {
                    '(' => {
                        match read_list(&mut chars) {
                            Ok(l) => {
                                v.push(l);
                                chars.next();
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    '0' ..= '9'|'-' => {
                        match read_num(&mut chars) {
                            Ok(num) => {
                                v.push(num);
                                chars.next();
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    'a' ..= 'z'|'A' ..= 'Z' => {
                        v.push(read_symbol(&mut chars));
                        chars.next();
                    },
                    '*' => {
                        match read_special(&mut chars) {
                            Ok(sp) => {
                                v.push(sp);
                                chars.next();
                            },
                            Err(e) => {
                                return Err(e);
                            }
                       }
                    },
                    ':' => {
                        v.push(read_keyword(&mut chars));
                        chars.next();
                    },
                    '\"' => {
                        v.push(read_string(&mut chars));
                        chars.next();
                    },
                    '\'' => {
                        match read_quote(&mut chars) {
                            Ok(q) => {
                                v.push(q);
                                chars.next();
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    ' ' => {
                        chars.next();
                    }
                    _ => return Err(LprpError::SyntaxError),
                }
            },
            None => return Ok(Token::List(v.to_vec())),
        }
    }
}
pub fn read<'a>(expr: &'a str) -> Result<Token, LprpError> {
    let mut chars = expr.chars().peekable();
    match read_expr(&mut chars) {
        Ok(ex) => {
            match ex {
                Token::List(l) => Ok(l[0].clone()),
                _ => Err(LprpError::ReadError),
            }
        },
        Err(e) => Err(e),
    }
}

#[test]
fn test_read() {
    let mut expr = "((1 -2.3) (*a* :b))".chars().peekable();
    assert_eq!(
        read_list(&mut expr),
        Ok(Token::List(vec![
            Token::List(vec![
                Token::Int(1),
                Token::Float(-2.3),
            ]),
            Token::List(vec![
                Token::Special("a".to_string()),
                Token::Keyword("b".to_string()),
            ])
        ]))
    );
}
