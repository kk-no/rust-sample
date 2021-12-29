use std::fs::read_to_string;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run_cat(path)
    }
}

fn run_cat(path: String) {
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => println!("{}", reason)
    }
}