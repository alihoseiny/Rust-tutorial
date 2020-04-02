use std::io;

fn main() {
    let mut user_input = String::new();
    let read_result = io::stdin().read_line(&mut user_input);
    match read_result {
        Ok(num_of_characters) => {
            println!("user input is: {}", user_input);
            println!("user input read result: {:?}", read_result);
        },
        Err(error) => println!("Error happened!")
    }
}
