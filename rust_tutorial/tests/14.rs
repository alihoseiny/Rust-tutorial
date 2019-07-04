extern crate core;

#[cfg(test)]
mod test14 {

    fn print_message(user_level: u8) {
        match user_level {
            0 => println!("Welcome dear admin."),
            1 => println!("Welcome back our best member."),
            2 => println!("Hello. Please register first."),
            _ => println!("Bad Input")
        }
    }

    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
        InternalServerError = 500
    }

    enum Colour {
        RGB(u16, u16, u16),
        Hex(String)
    }

    #[test]
    fn test_match_literal_syntax() {
        let user_inputs: [u8;5] = [0, 1, 2, 3, 4];
        for value in user_inputs.iter() {
            print_message(*value);
        }
    }

    struct RGB {
        red: u16,
        green: u16,
        blue: u16
    }

    struct RgbStruct {
        red: u16,
        green: u16,
        blue: u16
    }

    enum Colour2 {
        Transparent,
        RGB(RgbStruct),
        Hex(String),
        CMYK {cyan: f32, magenta: f32, yellow: f32, black: f32}
    }

    struct Foo(u8, u8);

    struct Point(u8, u8);

    fn print_point(Point(x, y): Point) {
        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_enum_matching() {
        let a = HttpStatus::Ok;
        match a {
            HttpStatus::Ok => println!("passed"),
            HttpStatus::InternalServerError => panic!("Pattern matching for enum values failed"),
            HttpStatus::NotFound => panic!("Pattern matching for enum values failed")
        }
    }

    #[test]
    fn test_enum_value_extraction() {
        let a = Colour::RGB(255, 123, 30);
        let b = Colour::Hex(String::from("ff7b1e"));
        match a {
            Colour::RGB(red, green, blue) => {
                assert_eq!(red, 255);
                assert_eq!(green, 123);
                assert_eq!(blue, 30);
            }
            Colour::Hex(hex) => panic!("Invalid extraction")
        }

        match b {
            Colour::RGB(red, green, blue) => panic!("Invalid extraction"),
            Colour::Hex(hex) => assert_eq!(hex, "ff7b1e")
        }
    }

    #[test]
    fn test_literal_and_variable_in_match() {
        let a = Colour::RGB(255, 0, 0);
            match a {
                Colour::RGB(255, 0, 0) => println!("RED Colour. Here is a completely different code."),
            Colour::RGB(red, green, blue) => {
                panic!("bad match");
            },
            Colour::Hex(hex) => panic!("bad match")
            }
    }

    #[test]
    fn test_range_in_pattern() {
        match 'a' {
            'A'...'Z' => panic!("bad match"),
            'a'...'z' => println!("is a lowercase english character."),
            _ => panic!("bad match")
        }
    }

    #[test]
    fn test_or_in_patterns() {
        let a = 12.0;
        match a {
            8.0...12.0 | 13.0...18.0 => println!("The store is open."),
            12.0...13.0 => panic!("bad match"),
            _ => panic!("bad match")
        }
    }

    #[test]
    fn test_value_extraction_from_tuple_by_pattern() {
        let rgb = (113, 0, 0);
        match rgb {
            (red, 0, 0) => println!("This colour is a gradient of red. red value: {}", red),
            (red, green, blue) => panic!("bad match")
        }
    }

    #[test]
    fn test_value_extraction_from_struct_by_pattern() {
        let r1 = RGB {
            red: 113,
            green: 0,
            blue: 0
        };
        match r1 {
            RGB {red: r, green: 0, blue: 0} => println!("This colour is a gradient of red. red value: {}", r),
            RGB {red, blue, green} => panic!("bad match")
        }
    }

    #[test]
    fn test_value_extraction_from_nested_struct_enum() {
        let rgb = Colour2::RGB(RgbStruct{
            red: 255,
            green: 255,
            blue: 255
        });
        match rgb {
            Colour2::Transparent => panic!("bad match"),
            Colour2::RGB(RgbStruct{red, green, blue}) => {
                assert_eq!(red, 255);
                assert_eq!(green, 255);
                assert_eq!(blue, 255);
            },
            Colour2::Hex(hex_value) => panic!("bad match"),
            Colour2::CMYK {cyan: c, magenta: m, yellow: y, black: k} => panic!("bad match")
        }
    }

    #[test]
    fn test_reference_matching() {
        let colours = [
            RgbStruct {
                red: 112,
                green: 0,
                blue: 0
            },
            RgbStruct {
                red: 123,
                green: 0,
                blue: 0
            }
        ];
        for rgb_reference in colours.iter() {
            match rgb_reference {
                &RgbStruct {red, blue: 0, green: 0} => println!("This is a kind of red colour."),
                RgbStruct {red, green, blue} => panic!("bad match")
            }
        }
    }

    #[test]
    fn test_pattern_guards() {
        let input = Foo(2, 9);
        match input {
            Foo(x, _) if x % 2 == 0 => println!("{} is an even number", x),
            Foo(x, y) => panic!("bad match")
        }
    }

    #[test]
    fn test_at_in_pattern() {
        let colour = RgbStruct {
            red: 50,
            blue: 20,
            green: 0
        };
        match colour {
            RgbStruct {red: r @ 0...100, green: 0, blue} => println!("This is my colour:rgb({}, 0, {})", r, blue),
            _ => panic!("bad match")
        }
    }

    #[test]
    fn test_pattern_in_if_let() {
        let hour: u8 = 10;
        if let  0...24 = hour {
            println!("a valid hour");
        } else {
            panic!("bad match");
        }
    }

    #[test]
    fn test_pattern_in_while_let() {
        let mut counter = 0;
        while let 0 | 1 | 2 | 3 = counter {
            counter += 1;
        }
        assert_eq!(4, counter);
    }

    #[test]
    fn test_pattern_in_let() {
        let (a, b, c) = (10, String::from("A String"), false);
        assert_eq!(a, 10);
        assert_eq!("A String", b);
        assert_eq!(c, false);
    }

    fn test_pattern_extraction_in_function_input_parameter() {
        print_point(Point(10, 10));
    }

}