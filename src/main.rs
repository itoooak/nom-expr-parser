use expr_parser::{naive, parenless};
use std::io::{stdin, stdout, Write};

macro_rules! parse_and_print {
    ($parser:expr, $name:literal, $input:expr) => {
        print!("[{}] ", $name);
        match $parser($input) {
            Ok((_, v)) => println!("result: {}", v),
            Err(e) => println!("error: {}", e.to_string()),
        };
    };
}

fn main() {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        parse_and_print!(naive::parse, "naive", &buf);
        parse_and_print!(parenless::parse, "parenless", &buf);
        stdout().flush().unwrap();
    }
}
