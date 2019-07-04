fn ignore_tuple(input_tuple: (u8, u8, u8, u8)) {
    match input_tuple {
        (0, _, _, val4) => println!("4th value: {}", val4),
        (_, val2, val3, _) => println!("second and third values are: {} and {}", val2, val3)
                                    }
}

fn main() {
    ignore_tuple((10, 12, 13, 14));
    ignore_tuple((0, 1, 2, 3));
}
