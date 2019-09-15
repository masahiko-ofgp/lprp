use lprp::reader::{read, Token};

#[test]
fn test_read() {
    let gv = "*global*".to_string();
    assert_eq!(read(&gv), Ok(Token::Symbol("*global*".to_string())));

    let expr = "(cons (cons -1 2.0) (cons \"Hello, world!!\" nil))".to_string();
    assert_eq!(
        read(&expr),
        Ok(Token::List(vec![
                       Token::Symbol("cons".to_string()),
                       Token::List(vec![
                                   Token::Symbol("cons".to_string()),
                                   Token::Int(-1),
                                   Token::Float(2.0)
                       ]),
                       Token::List(vec![
                                   Token::Symbol("cons".to_string()),
                                   Token::Str("Hello, world!!".to_string()),
                                   Token::Nil
                       ])
        ]))
        );
}
