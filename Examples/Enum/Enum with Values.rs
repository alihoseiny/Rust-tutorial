enum Colour {
    RGB(u16, u16, u16),
    Hex(String)
}

fn main() {
    let rgb_colour = Colour::RGB(255, 0, 0);
    let hex_colour = Colour::Hex(String::from("ff0000"));
}
