pub struct Config {
    pub query: String, 
    pub file_path: String, 
}

impl Config {

    pub fn build(args: &[String]) -> Result<Self, &'static str> {

        // Error Check
        if args.len() < 3 {
            return Err("Missing arguements. Expecting 2 or more paramters.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(
            Self {query, file_path}
        )
    }
}