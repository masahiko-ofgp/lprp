use lprp::reader::{Token, read};

#[test]
fn test_token() {
    let num = Token::Num(12);
    let st = Token::Str("abc".to_string());
    let list = Token::List(vec![Token::Num(1), Token::Str("a".to_string())]);

    assert_eq!(num.getn(), Some(12));
    assert_eq!(st.gets(), Some("abc".to_string()));
    assert_eq!(
        list.getv(),
        Some(vec![Token::Num(1), Token::Str("a".to_string())])
        );
}

#[test]
fn test_read() {
    let num = "123".to_string();
    let st = "hello".to_string();
    let cons = "(cons 1 (cons 2 nil))".to_string();

    assert_eq!(
        read(&num),
        Token::Num(123)
        );
    assert_eq!(
        read(&st),
        Token::Str("hello".to_string())
        );
    assert_eq!(
        read(&cons),
        Token::List(vec![
                    Token::Str("cons".to_string()),
                    Token::Num(1),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Num(2),
                                Token::Str("nil".to_string())
                    ]),
        ])
        );
}
