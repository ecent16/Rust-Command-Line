
pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = String::from("duct");
        let contents = String::from("\
            Rust: 
            safe, fast, productive. 
            Pick three.
        ");

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

}