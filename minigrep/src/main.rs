use std::env; // Module

mod read;
use read::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // Search parameter
    let file_path = &args[2]; // File source

    println!("Searching for '{}'\n", query);

    read_file(file_path);

}
