fn print_rgb_tuple(rgb: (u16, u16, u16)) {
    match rgb {
        (red, 0, 0) => println!("This colour is a gradient of red. red value: {}", red),
        (red, green, blue) => println!("rgb({}, {}, {})", red, green, blue)
                                    }
}

fn main() {
    let r1 = (113, 0, 0);
    let r2 = (123, 221, 0);
    print_rgb_tuple(r1);
    print_rgb_tuple(r2);
}
