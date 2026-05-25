
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust: 
            safe, fast, productive. 
            Pick three.
            Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = String::from("\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.
        ");

        assert_eq!(
            vec!["Rust", "Trust Me."], 
            search_case_insensitive(&query, &contents)
        );
    }

}