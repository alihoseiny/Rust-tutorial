use std::io;
use std::fs;

fn main() {
    let mut path= String::new();
    loop {
        path.clear();
        get_file_path(&mut path);

        let trimmed_path = path.trim();
        let file_reading_result = read_file(trimmed_path);

        match file_reading_result {
            Ok(file_content) => {
                println!("file content:\n {}", file_content);
                return;
            }
            Err(error) => {
                println!("{}", error);
                println!("Please try again.");
            }
        }

    }
}

fn get_file_path(user_input: &mut String) {
    println!("Input file path:");
    match io::stdin().read_line(user_input) {
        Err(_) => println!("Error happened in getting input from user"),
        _ => {}
    }
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let read_result = fs::read_to_string(file_name);
    match read_result {
        Ok(content) => Ok(content),
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => Err(error),
                _ => panic!("Something bad happened. :(")
            }
        }
    }
}
