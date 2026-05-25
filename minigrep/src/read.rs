use std::{error::Error, fs};

use crate::config::Config;
use minigrep::{search, search_case_insensitive};

pub fn read_file(config: &Config) -> Result<(), Box<dyn Error>> {

    println!("In file {}", &config.file_path);

    let contents = fs::read_to_string(&config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}