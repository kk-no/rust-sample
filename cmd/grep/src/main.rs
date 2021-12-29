use structopt::StructOpt;
use std::fs::read_to_string;

#[derive(StructOpt)]
#[structopt(name = "grep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "PATHS")]
    paths: Vec<String>,
}

fn main() {
    run(GrepArgs::from_args());
}

fn run(args: GrepArgs) {
    for path in args.paths.iter() {
        match read_to_string(path) {
            Ok(content) => grep(&content, &args.pattern),
            Err(reason) => println!("{}", reason),
        }
    }
}

fn grep(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line)
        }
    }
}
