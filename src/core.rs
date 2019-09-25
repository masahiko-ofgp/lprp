// Copyright 2019 Masahiko Hamazawa
//
// Licensed under the MIT license <LICENSE or
//  http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, on distributed except
//  according to those terms.

use crate::reader::{read, Token};


// ***** eq *****
pub fn eq(lhs: &Token, rhs: &Token) -> bool {
    lhs == rhs
}

#[test]
fn test_eq() {
    let list_1 = read("'(1 2 3)").unwrap();
    assert_eq!(eq(
            &list_1,
            &Token::Quote(Box::new(Token::List(vec![
                                               Token::Int(1),
                                               Token::Int(2),
                                               Token::Int(3),
            ])))),
            true
            );

    let sym_1 = read("cons").unwrap();
    let sym_2 = read("conj").unwrap();
    assert_eq!(eq(&sym_1, &sym_2), false);
}

// ***** atom *****
pub fn atom(tk: &Token) -> bool {
    match tk {
        Token::List(l) => {
            if l.len() == 0_usize { true } else { false }
        },
        Token::Quote(q) => atom(&(*q)),
        _ => true
    }
}

#[test]
fn test_atom() {
    let num = read("123").unwrap();
    assert_eq!(atom(&num), true);

    let nil = read("'()").unwrap();
    assert_eq!(atom(&nil), true);

    let list = read("'(1 2 3)").unwrap();
    assert_eq!(atom(&list), false);
}

// ***** car *****
pub fn car(tk: &Token) -> Option<&Token> {
    match tk {
        Token::List(l) => {
            let (hd, _) = l.split_first().unwrap();
            Some(hd)
        },
        Token::Quote(q) => car(&(*q)),
        _ => None,
    }
}

#[test]
fn test_car() {
    let list = read("(cons 1 2)").unwrap();
    assert_eq!(
        car(&list),
        Some(&Token::Symbol("cons".to_string()))
        );

    let quote_list = read("'(1 2 3)").unwrap();
    assert_eq!(
        car(&quote_list),
        Some(&Token::Int(1))
        );
}

// ***** cdr *****
pub fn cdr(tk: &Token) -> Option<Token> {
    match tk {
        Token::List(l) => {
            let (_, tl) = l.split_first().unwrap();
            Some(Token::List(tl.to_vec()))
        },
        Token::Quote(q) => cdr(&(*q)),
        _ => None
    }
}

#[test]
fn test_cdr() {
    let list = read("(cons 1 2)").unwrap();

    assert_eq!(
        cdr(&list),
        Some(Token::List(vec![Token::Int(1), Token::Int(2)]))
    );

    let quote_list = read("'(1 2 3)").unwrap();

    assert_eq!(
        cdr(&quote_list),
        Some(Token::List(vec![Token::Int(2), Token::Int(3)]))
        );
}

// Check whether List's car is symbol or not.
pub fn is_car_sym(tk: &Token) -> bool {
    match car(tk) {
        Some(t) => {
            match t {
                Token::Symbol(_) => true,
                _ => false,
            }
        },
        None => false
    }
}

#[test]
fn test_is_car_sym() {
    let list = read("(cons 1 nil)").unwrap();
    assert!(is_car_sym(&list));

    let e = read("'(1 2 3)").unwrap();
    assert!(!is_car_sym(&e));

    let quote = read("'(cons 1.2 \"Hello\")").unwrap();
    assert!(is_car_sym(&quote));
}

// If List's car is symbol, return symbol.
pub fn get_sym(tk: &Token) -> Option<&Token> {
    if is_car_sym(tk) {
        Some(car(tk).unwrap())
    } else {
        None
    }
}

#[test]
fn test_get_sym() {
    let list = read("(format t \"Hello\")").unwrap();

    assert_eq!(
        get_sym(&list),
        Some(&Token::Symbol("format".to_string()))
        );

    let e = read("'(1 2 3)").unwrap();
    assert_eq!(get_sym(&e), None);
}

pub fn get_args(tk: &Token) -> Option<Token> {
    if is_car_sym(tk) {
        Some(cdr(tk).unwrap())
    } else {
        None
    }
}

#[test]
fn test_get_args() {
    let list = read("(car '(1 2))").unwrap();

    assert_eq!(
        get_args(&list),
        Some(Token::List(vec![
                         Token::Quote(Box::new(
                                 Token::List(vec![
                                             Token::Int(1),
                                             Token::Int(2)
                                 ])))
        ])));
}

pub fn eval<'a>(exp: &'a str) -> Token {
    let token = read(exp).unwrap();
    
    if atom(&token) {
        return token;
    } else {
        // TODO: Temporary return value
        return Token::Nil;
    }
}
