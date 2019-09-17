enum Result<T> {
    OK(T),
    Err(())
}

fn main() {
    let mut my_point = Point::<u8> {
        x: 10,
        y: 12
    };

    let that_is_ok = Result::OK::<Point<u8>>(my_point);

    match that_is_ok {
        Result::OK(Point {x, y}) => println!("x: {}, y: {}", x, y),
        Result::Err(_) => println!("An Error happened"),
        _ => {}
    };
}
