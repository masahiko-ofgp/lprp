use crate::reader::{Token, ReaderError, read};

// Check Token::List or not.
fn is_list(token: &Token) -> bool {
    match token {
        Token::List(_) => true,
        _ => false
    }
}

// Check Token::Sym or not.
fn is_sym(token: &Token) -> bool {
    match token {
        Token::Sym(_) => true,
        _ => false,
    }
}


pub fn eval<'a>(sexp: &'a str) -> Result<Token, ReaderError> {
    match read(&sexp) {
        Ok(tk) => {
            if is_list(&tk) {
                Ok(tk)
            } else {
                if is_sym(&tk) {
                    Err(ReaderError::SyntaxError)
                } else {
                    Ok(tk)
                }
            }
        },
        Err(e) => Err(e),
    }
}
