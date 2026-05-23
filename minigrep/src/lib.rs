
pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {

    let  mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    if result.len() < 1 {
        result.push("No results found");
    }

    result
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