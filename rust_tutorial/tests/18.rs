#[cfg(test)]
mod test18 {
    use std::mem;

    struct Point<Type> {
        x: Type,
        y: Type
    }

    impl<Type> Point<Type> {
        fn swap_coordinates(&mut self) {
            mem::swap(&mut self.x, &mut self.y);
        }
    }

    fn swap_point<T>(original_point: Point<T>) -> Point<T> {
        let Point {x, y} = original_point;
        return Point {
            x: y,
            y: x
        }
    }

    enum Result<T> {
        OK(T),
        Err(())
    }

    #[test]
    fn test_type_casting_u8_to_i32() {
        let a: u8 = 10;
        let b: i32 = a as i32;
        assert_eq!(b, 10);
    }

    #[test]
    fn test_type_casting_signed_to_unsigned() {
        let a: i8 = -10;
        let b: u8 = a as u8;
        assert_eq!(b, 246);
    }

    #[test]
    fn test_type_casting_overflow() {
        let a: u64 = 1_000_000;
        let b: u8 = a as u8;
        assert_eq!(b, 64);
    }

    fn test_type_casting_bool_to_number() {
        let yes = true;
        let no = false;
        assert_eq!(yes as u8, 1);
        assert_eq!(no as u8, 0);
    }

    #[test]
    fn test_generic_swap_function() {
        let my_point = Point::<u8> {
            x: 10,
            y: 12
        };

        let swapped_point = swap_point(my_point);

        assert_eq!(swapped_point.x, 12);
        assert_eq!(swapped_point.y, 10);
    }

    #[test]
    fn test_generic_struct_generic_method_swap() {
        let mut my_point = Point::<u8> {
            x: 10,
            y: 12
        };
        my_point.swap_coordinates();
        assert_eq!(my_point.x, 12);
        assert_eq!(my_point.y, 10);
    }

    #[test]
    fn test_generic_enum_syntax() {
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
}