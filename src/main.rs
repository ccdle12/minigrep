use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Read contents: {}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 'static is a lifetime that lives the whole duration of a program. Useful for error messages
    // but should be used carefully.
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Make a deep copy of the string in the vector.
        // Clone is very inefficient.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
