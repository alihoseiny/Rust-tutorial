enum Colour {
    RGB(u16, u16, u16),
    Hex(String)
}

fn print_colour(colour_value: Colour) {
    match colour_value {
        Colour::RGB(255, 0, 0) => println!("RED Colour. Here is a completely different code."),
        Colour::RGB(red, green, blue) => {
            println!("The colour is in rgb format. red value: {}, green value: {}, blue value: {}", red, green, blue);
        },
        Colour::Hex(hex) => println!("The colour is in hex format: #{}", hex)
    }
}

fn main() {
    let a = Colour::RGB(255, 123, 30);
    let b = Colour::Hex(String::from("ff7b1e"));
    let red = Colour::RGB(255, 0, 0);
    print_colour(a);
    print_colour(b);
    print_colour(red);
}
