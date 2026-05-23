use std::{error::Error, fs};

use crate::config::Config;
use minigrep::search;

pub fn read_file(config: &Config) -> Result<(), Box<dyn Error>> {

    println!("In file {}", &config.file_path);

    let contents = fs::read_to_string(&config.file_path)?;

    //println!("With text:\n{}", contents);
    for i in search(&config.query, &contents) {
        println!("{i}");
    }

    Ok(())
}