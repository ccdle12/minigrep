use std::env;
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

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong when reading the file");

    println!("Read contents: {}", contents);
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
