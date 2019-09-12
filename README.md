# lprp

lprp ('l'eft 'p'aren 'r'ight 'p'aren) is simple S-expression reader.

- [x] Int
- [x] Float
- [x] Symbol
- [x] Keyword
- [x] Global variable
- [x] List
- [x] Quote
- [x] Nil

```
use lprp::reader::read;

fn main() {
    let sexp = "(cons (cons 1 2.0) (cons \"Hello world!!\" nil))".to_string();

    let result = read(&sexp).unwrap();

    assert_eq!(
        result,
        Token::List(vec![
                    Token::Symbol("cons".to_string()),
                    Token::List(vec![
                                Token::Symbol("cons".to_string()),
                                Token::Int(1),
                                Token::Float(2.0)
                    ]),
                    Token::List(vec![
                                Token::Str("cons".to_string()),
                                Token::Str("Hello, world!!"),
                                Token::Nil
                    ])
        ])
    );
}
```
