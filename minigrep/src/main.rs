use minigrep::Config;

use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: minigrep <needle> <filename>");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
