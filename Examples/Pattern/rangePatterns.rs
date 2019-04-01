fn print_character_type(character: char) {
    match character {
        'a'...'z' => println!("{} is a lowercase english character.", character),
        'A'...'Z' => println!("{} is a uppercase english character.", character),
        '0'...'9' => println!("{} is an english digit.", character),
        _ => println!("This character is not an english character.")
                                                    }
}

fn main() {
    let characters = ['X', 'y', '4', 'س'];
    for ch in characters.iter() {
        print_character_type(*ch);
    }
}
