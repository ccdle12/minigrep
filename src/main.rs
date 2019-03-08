use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    // We are using this way to catch the error as opposed to unwrap_or_else because run does not
    // return any values besides an error.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}
