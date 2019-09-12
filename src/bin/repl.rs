use lprp::reader::read;
use std::error::Error;
use std::io::{self, Write};

fn main() {
    loop {
        let mut s = String::new();
        print!("LPRP>> ");
        io::stdout().flush().expect("Couldn't flush stdout.");
        io::stdin().read_line(&mut s).expect("Failed.");

        if (s.starts_with("quit"))||(s.starts_with("(quit)")) {
            break;
        } else {
            let s2 = &s.trim_end_matches("\n");
            match read(&s2) {
                Ok(r) => {
                    println!("{:?}", r);
                },
                Err(e) => {
                    eprintln!("{:?}", e.description());
                    break;
                }
            }
        }
    }
}
