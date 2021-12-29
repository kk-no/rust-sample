use std::fs::read_to_string;

fn main() {
    let path = std::env::args().nth(1);
    let pattern = std::env::args().nth(2);

    match (path, pattern) {
        (Some(path), Some(pattern)) => run(path, pattern),
        _ => println!("no argument specified.")
    }
}

fn run(path: String, pattern: String) {
    match read_to_string(path) {
        Ok(content) => grep(content, pattern),
        Err(reason) => println!("{}", reason)
    }
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line)
        }
    }
}