enum Colour {
    RGB,
    Hex
}

fn main() {
    let a = Colour::RGB;
    let b = Colour::Hex;
    println!("RGB value in memory: {}", a as u8);
    println!("Hex value in memory: {}", b as u8);
}
