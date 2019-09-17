use std::mem;

struct Point<T> {
    x: T,
    y: T
}

impl<Type> Point<Type> {
    fn swap_coordinates(&mut self) {
        mem::swap(&mut self.x, &mut self.y);
    }
}

fn main() {
    let pointer_u8 = Point::<u8> {
        x: 10,
        y: 12
    };

    let float_pointer = Point::<f32> {
        x: 0.0,
        y: 666.32
    };

    let detect_my_type = Point {
        x: 10,
        y: 11
    };

    let mut point_integer = Point {
        x: 10,
        y: 11
    };

    println!("Before swapping x: {} y: {}", point_integer.x, point_integer.y);

    point_integer.swap_coordinates();

    println!("After swapping x: {} y: {}", point_integer.x, point_integer.y);

    let my_point2 = Point::<u8> {
        x: 10,
        y: 12
    };

    let swapped_point = swap_point::<u8>(my_point2);
    println!("swapped point x: {}, y: {}", swapped_point.x, swapped_point.y);
}
