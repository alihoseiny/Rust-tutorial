#[cfg(test)]
mod test13 {

    enum Colour {
        RGB,
        Hex
    }

    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
        InternalServerError = 500
    }

    struct ColourStruct {
        colour_type: Colour,
        value: String
    }

        enum ColourWithValue {
        RGB(u16, u16, u16),
        Hex(String)
    }

    enum ColourWithStruct {
        RGB {red: u16, green: u16, blue: u16},
        Hex(String)
    }

    #[test]
    fn test_enum_declaration() {
        let a = Colour::RGB;
    }

    #[test]
    fn test_enum_values_memory_representation() {
        let a = Colour::RGB;
        let b = Colour::Hex;
        assert_eq!(a as u8, 0);
        assert_eq!(b as u8, 1);
    }

    #[test]
    fn test_enum_value_number_declaration() {
        let a = HttpStatus::Ok;
        let b = HttpStatus::NotFound;
        let c = HttpStatus::InternalServerError;
        assert_eq!(a as u8, 200);
        assert_eq!(b as u16, 404);
        assert_eq!(c as u16, 500);
    }

    #[test]
    ///
    /// Only tests syntax.
    fn test_enum_in_struct() {
        let rgb_colour = ColourStruct {
            colour_type: Colour::RGB,
            value: String::from("(255, 0, 0)")
        };

        let hex_colour = ColourStruct {
            colour_type: Colour::Hex,
            value: String::from("ff0000")
        };
    }

    #[test]
    ///
    /// Only tests the syntax
    fn test_storing_value_in_enum() {
        let rgb_colour = ColourWithValue::RGB(255, 0, 0);
        let hex_colour = ColourWithValue::Hex(String::from("ff0000"));
    }

    #[test]
    ///
    /// Only tests the syntax
    fn test_struct_in_tuple() {
        let rgb_colour = ColourWithStruct::RGB {
            blue: 0,
            red: 255,
            green: 0
        };

        let hex_colour = ColourWithStruct::Hex(String::from("ff0000"));
    }
}