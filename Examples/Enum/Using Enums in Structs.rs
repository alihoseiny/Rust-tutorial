enum Colour {
    RGB,
    Hex
}

struct ColourStruct {
    colour_type: Colour,
    value: String
}

fn main() {
    let rgb_colour = ColourStruct {
        colour_type: Colour::RGB,
        value: String::from("(255, 0, 0)")
    };
    
    let hex_colour = ColourStruct {
        colour_type: Colour::Hex,
        value: String::from("ff0000")
    };
}
