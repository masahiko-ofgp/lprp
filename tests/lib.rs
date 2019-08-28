use lprp::reader::{Token, read};

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
