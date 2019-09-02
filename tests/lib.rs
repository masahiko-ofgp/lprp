use lprp::reader::{Token, Symbol, read};

#[test]
fn test_token() {
    let st = Token::Str("abc".to_string());
    let list = Token::List(vec![Token::Int(1), Token::Str("a".to_string())]);

    assert_eq!(st.gets(), Some("abc".to_string()));

    assert_eq!(
        list.getl(),
        Some(vec![Token::Int(1), Token::Str("a".to_string())])
        );
}

#[test]
fn test_read() {
    let num = "123".to_string();
    let st = "hello".to_string();
    let cons = "(cons -1 (cons 2.0 nil))".to_string();

    assert_eq!(
        read(&num),
        Ok(Token::Int(123))
        );
    assert_eq!(
        read(&st),
        Ok(Token::Str("hello".to_string()))
        );
    assert_eq!(
        read(&cons),
        Ok(Token::List(vec![
                    Token::Sym(Box::new(Symbol::Cons)),
                    Token::Int(-1),
                    Token::List(vec![
                                Token::Sym(Box::new(Symbol::Cons)),
                                Token::Float(2.0),
                                Token::Nil
                    ]),
        ]))
        );
}
