use lprp::reader::{Token, LprpError, read};

#[test]
fn test_token() {
    let st = Token::Str("abc".to_string());
    let list = Token::List(vec![Token::Int(1), Token::Str("a".to_string())]);

    assert_eq!(st.gets(), Ok("abc".to_string()));

    assert_eq!(
        list.getl(),
        Ok(vec![Token::Int(1), Token::Str("a".to_string())])
        );
}

#[test]
fn test_car_cdr() {
    let list = Token::List(vec![
                           Token::Int(1),
                           Token::List(vec![Token::Int(2), Token::Nil])
    ]);
    assert_eq!(
        list.car(),
        Ok(Token::Int(1)),
        );
    assert_eq!(
        list.cdr(),
        Ok(Token::List(vec![
                       Token::List(vec![Token::Int(2), Token::Nil
                       ])
        ])));
    assert_eq!(
        list.cdr().unwrap().cdr().unwrap(),
        Token::List(vec![Token::Nil]),
        );

    let num = Token::Float(1.2);

    assert_eq!(
        num.car(),
        Err(LprpError::SyntaxError)
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
                    Token::Str("cons".to_string()),
                    Token::Int(-1),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Float(2.0),
                                Token::Nil
                    ]),
        ]))
        );
        

    let sexp = "(cons (cons 1 2.0) (cons -3 nil))".to_string();

    let result = read(&sexp).unwrap();

    assert_eq!(
        result,
        Token::List(vec![
                    Token::Str("cons".to_string()),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Int(1),
                                Token::Float(2.0)
                    ]),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Int(-3),
                                Token::Nil
                    ])
        ])
    );
}
