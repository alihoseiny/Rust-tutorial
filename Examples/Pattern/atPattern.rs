struct RgbStruct {
    red: u16,
    green: u16,
    blue: u16
}

fn extract_and_match(colour: RgbStruct) {
    match colour {
        RgbStruct {red: r @ 0...100, green: 0, blue} => println!("This is my colour: rgb({}, 0, {})", r, blue),
        _ => println!("Not desired colour.")
                                    }
}

fn main() {
    let colour1 = RgbStruct {
        red: 120,
        green: 0,
        blue: 255
    };

    let colour2 = RgbStruct {
        red: 50,
        blue: 20,
        green: 0
    };

    extract_and_match(colour1);
    extract_and_match(colour2);
}
