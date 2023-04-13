use std::env;
use std::process;

use minigrep_rust::Config;

fn main() {
    // Get the list of command line arguments from the program
    //   .collect() - turn an iterator into an actual collection of a particular type
    let args: Vec<String> = env::args().collect();

    // Get a reference to the 2nd/3rd command line args, which are the search query
    // and file path to search in, respectively.
    //   .unwrap_or_else() = Since we want the value AND to check for errors, we use this
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Here, since we ONLY care about if there's an error, and not the actual return value,
    // we can use the "if let" construct to just check for the Error case, and report the error
    if let Err(e) = minigrep_rust::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
