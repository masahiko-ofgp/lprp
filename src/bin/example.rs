use lprp::reader::read;
use std::env;

fn main() {
    let argv = env::args().collect::<Vec<_>>();

    if argv.len() != 2 {
        eprintln!("ERROR: Incorrect number of arguments");
    } else {
        println!("{:?}", read(&argv[1]));
    }
}
