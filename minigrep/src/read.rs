use std::fs;

fn read_file(file_path: String) {
    
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should be able to read file");

    println!("With text:\n{}", contents);
}