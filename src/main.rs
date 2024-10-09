use expr_parser::{naive, parenless};
use nom::error::convert_error;
use std::io::{stdin, stdout, Write};

macro_rules! parse_and_print {
    ($parser:expr, $name:literal, $input:expr) => {
        print!("[{}] ", $name);
        match $parser($input) {
            Ok((_, v)) => println!("result: {}", v),
            Err(nom::Err::Error(e)) => println!("error: {}", convert_error($input.as_str(), e)),
            _ => {}
        };
    };
}

fn main() {
    loop {
        let mut buf = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        parse_and_print!(naive::parse, "naive", &buf);
        parse_and_print!(parenless::parse, "parenless", &buf);
        stdout().flush().unwrap();
    }
}
