struct RgbStruct {
    red: u16,
    green: u16,
    blue: u16
}


fn print_number(n: u16) {
    println!("number is: {}", n);
}


fn main() {
    let colours = [
        RgbStruct {
            red: 112,
            green: 0,
            blue: 0
        },
        RgbStruct {
            red: 123,
            green: 124,
            blue: 8
        },
        RgbStruct {
            red: 0,
            green: 41,
            blue: 223
        }
    ];
    
    for rgb_reference in colours.iter() {
        match rgb_reference {
            &RgbStruct {red, blue: 0, green: 0} => {
                println!("This is a kind of red colour.");
                print_number(red);
            },
            RgbStruct {red, green, blue} => println!("rgb({}, {}, {})", red, green, blue)
        }
    }
}
