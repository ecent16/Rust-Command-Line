use std::{error::Error, fs};

use crate::config::Config;

pub fn read_file(config: &Config) -> Result<(), Box<dyn Error>> {

    println!("In file {}", &config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect("Should be able to read file");

    println!("With text:\n{}", contents);

    Ok(())
}