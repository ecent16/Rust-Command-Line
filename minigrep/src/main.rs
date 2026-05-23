use std::{env, process}; // Module

mod read;
use read::read_file;

mod config;
use config::Config;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'\n", config.query);

    // Handle the error condition for read_file
    if let Err(e) = read_file(&config) {
        println!("Application error: {}", e); 
        process::exit(1);
    };

}

