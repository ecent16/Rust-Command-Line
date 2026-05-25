use std::env;

pub struct Config {
    pub query: String, 
    pub file_path: String, 
    pub ignore_case: bool, 
}

impl Config {

    pub fn build(args: &[String]) -> Result<Self, &'static str> {

        // Error Check
        if args.len() < 3 {
            return Err("Missing arguements. Expecting 2 or more paramters.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(
            Self {query, file_path, ignore_case}
        )
    }
}