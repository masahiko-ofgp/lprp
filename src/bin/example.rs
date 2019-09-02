use lprp::eval::eval;
use std::env;
use std::error::Error;


fn main() {
    let argv = env::args().collect::<Vec<_>>();

    if argv.len() != 2 {
        eprintln!("ERROR: Incorrect number of arguments");
    } else {
        match eval(&argv[1]) {
            Ok(ev) => {
                println!("{:?}", ev);
            },
            Err(e) => eprintln!("ERROR: {}", e.description()),
        }
    }
}
