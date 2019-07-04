struct RGB {
    red: u16,
    green: u16,
    blue: u16
}

fn print_rgb_struct(rgb: RGB) {
    match rgb {
        RGB {red: r, green: 0, blue: 0} => println!("This colour is a gradient of red. red value: {}", r),
        RGB {red, blue, green} => println!("rgb({}, {}, {})", red, green, blue)
                                    }
}

fn main() {
    let r1 = RGB {
        red: 113,
        green: 0,
        blue: 0
    };

    let r2 = RGB {
        red: 123,
        green: 221,
        blue: 0
    };

    print_rgb_struct(r1);
    print_rgb_struct(r2);
}
