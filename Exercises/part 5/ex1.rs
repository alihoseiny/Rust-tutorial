fn main() {
    let while_condition: bool;
    while_condition = true; // Creates a infinite loop. But it's just an example and we don't care.
    loop{
        if while_condition {
            println!("loop code");
        }
        else {
            break;
        }
    }
}

