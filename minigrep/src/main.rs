use std::{env, process}; // Module

mod read;
use read::read_file;

mod config;
use config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let (query, file_path) = parse_config(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'\n", config.query);

    read_file(&config.file_path);

}

