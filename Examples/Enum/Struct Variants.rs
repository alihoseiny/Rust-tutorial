enum Colour {
    RGB {red: u16, green: u16, blue: u16},
    Hex(String)
}

fn main() {
    let rgb_colour = Colour::RGB {
        blue: 0,
        red: 255,
        green: 0
    };
    
    let hex_colour = Colour::Hex(String::from("ff0000"));
}

