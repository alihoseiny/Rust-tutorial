use std::io;
use std::fs;
use std::process::exit;

fn main() {
    let mut path = String::new();
    get_file_path(&mut path);
    let trimmed_path = path.trim();
    let file_content = read_file(trimmed_path);
    println!("file content:\n {}", file_content);
}

fn get_file_path(user_input: &mut String) {
    println!("Input file path:");
    match io::stdin().read_line(user_input) {
        Err(_) => println!("Error happened in getting input from user"),
        _ => {}
    }
}

fn read_file(file_name: &str) -> String {
    let read_result = fs::read_to_string(file_name);
    match read_result {
        Ok(content) => content,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    println!("The {} file not found. Please re-run the program and try another file.", file_name);
                    exit(0);
                }
                _ => panic!("Something bad happened. :(")
            }
        }
    }
}
