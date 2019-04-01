struct RgbStruct {
    red: u16,
    green: u16,
    blue: u16
}

enum Colour {
    Transparent,
    RGB(RgbStruct),
    Hex(String),
    CMYK {cyan: f32, magenta: f32, yellow: f32, black: f32}
}

fn print_nested_structures(colour: Colour) {
    match colour {
        Colour::Transparent => println!("This is Transparent! You can not see anything"),
        Colour::RGB(RgbStruct{red, green, blue}) => {
            println!("Colour is in rgb format: ({}, {}, {})", red, green, blue)
        },
        Colour::Hex(hex_value) => println!("The colour in the hex format: #{}", hex_value),
        Colour::CMYK {cyan: c, magenta: m, yellow: y, black: k} => {
            println!("Colour is in cmyk format: ({}, {}, {}, {})", c, m, y, k)
        }
    }
}

fn main() {
    let t = Colour::Transparent;
    let rgb = Colour::RGB(RgbStruct{
        red: 255,
        green: 255,
        blue: 255
    });
    
    let hex = Colour::Hex(String::from("ffffff"));

    let cmyk = Colour::CMYK {
        cyan: 0.0,
        magenta: 0.0,
        yellow: 0.0,
        black: 0.0
    };
    
    print_nested_structures(t);
    print_nested_structures(rgb);
    print_nested_structures(hex);
    print_nested_structures(cmyk);
}
