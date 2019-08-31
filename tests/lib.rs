use lprp::reader::{Token, read};

#[test]
fn test_token() {
    let st = Token::Str("abc".to_string());
    let list = Token::List(vec![Token::Str("1".to_string()), Token::Str("a".to_string())]);

    assert_eq!(st.gets(), Some("abc".to_string()));

    assert_eq!(
        list.getl(),
        Some(vec![Token::Str("1".to_string()), Token::Str("a".to_string())])
        );
}

#[test]
fn test_read() {
    let num = "123".to_string();
    let st = "hello".to_string();
    let cons = "(cons 1 (cons 2 nil))".to_string();

    assert_eq!(
        read(&num),
        Ok(Token::Str("123".to_string()))
        );
    assert_eq!(
        read(&st),
        Ok(Token::Str("hello".to_string()))
        );
    assert_eq!(
        read(&cons),
        Ok(Token::List(vec![
                    Token::Str("cons".to_string()),
                    Token::Str("1".to_string()),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Str("2".to_string()),
                                Token::Str("nil".to_string())
                    ]),
        ]))
        );
}
