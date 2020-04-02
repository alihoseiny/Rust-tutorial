use std::io;

fn get_file_path(user_input: &mut String) {
    println!("Input file path:");
    io::stdin().read_line(user_input).expect("We are unable to get input from the standard input.");
}
