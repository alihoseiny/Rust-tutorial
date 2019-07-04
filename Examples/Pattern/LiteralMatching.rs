fn print_message(user_level: u8) {
    match user_level {
        0 => println!("Welcome dear admin."),
        1 => println!("Welcome back our best member."),
        2 => println!("Hello. Please register first."),
        _ => println!("Bad Input")
    }
}

fn main() {
    let user_inputs: [u8;5] = [0, 1, 2, 3, 4];
    for value in user_inputs.iter() {
        print_message(*value);
    }
}
