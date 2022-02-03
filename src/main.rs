use std::{ env, process };
use minigrep::utilities;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = utilities::parse_config(&args).unwrap_or_else(|why| {
        eprintln!("Problem parsing arguments: {} (exiting process with code 1...)", why);
        process::exit(1);
    });

    if let Err(why) = utilities::execute(config) {
        eprintln!("An error occurred: {} (exiting process with code 1...)", why);
        process::exit(1);
    };
}
