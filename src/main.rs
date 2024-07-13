use expr_parser::naive;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        match naive::parse(&buf) {
            Ok((_, v)) => println!("result: {v}"),
            Err(e) => println!("error: {}", e.to_string()),
        }
        stdout().flush().unwrap();
    }
}
