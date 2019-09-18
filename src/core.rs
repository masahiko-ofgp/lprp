use crate::reader::Token;
#[cfg(test)]
use crate::reader::read;

// ***** eq *****
pub fn eq(lhs: &Token, rhs: &Token) -> bool {
    lhs == rhs
}

#[test]
fn test_eq() {
    let list_1 = read("'(1 2 3)").unwrap();
    let list_2 = read("'(1 2 3)").unwrap();
    assert_eq!(eq(&list_1, &list_2), true);

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
    assert_eq!(car(&list), Some(&Token::Symbol("cons".to_string())));

    let quote_list = read("'(1 2 3)").unwrap();
    assert_eq!(car(&quote_list), Some(&Token::Int(1)));
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
