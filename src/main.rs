use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[0];
    let filename = &args[1];

    println!("Searching for {}", query);
    println!("In File {}", filename);
}
