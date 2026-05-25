use std::{env, process}; // Module

mod read;
use read::read_file;

mod config;
use config::Config;

fn main() {

    let args: Vec<String> = env::args().collect(); // Command line input

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err); // Error message print
        process::exit(1);
    });

    // Handle the error condition for read_file
    if let Err(e) = read_file(&config) {
        eprintln!("Application error: {}", e); // Error message print
        process::exit(1);
    };

}

