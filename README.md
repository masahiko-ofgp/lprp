# lprp

lprp ('l'eft 'p'aren 'r'ight 'p'aren) is simple S-expression reader.

- [x] Int
- [x] Float
- [x] Symbol
- [x] List
- [x] Quote
- [x] Nil
- [x] T

```
use lprp::reader::{read, Token};

fn main() {
    let sexp = "(cons (cons 1 2.0) (cons \"Hello world!!\" nil))".to_string();

    assert_eq!(
        read(&sexp),
        Ok(Token::List(vec![
                    Token::Symbol("cons".to_string()),
                    Token::List(vec![
                                Token::Symbol("cons".to_string()),
                                Token::Int(1),
                                Token::Float(2.0)
                    ]),
                    Token::List(vec![
                                Token::Symbol("cons".to_string()),
                                Token::Str("Hello, world!!"),
                                Token::Nil
                    ])
        ]))
    );
}
```
