#[cfg(test)]
mod test2 {
    #[test]
    fn test_numeric_types_var_declaration() {
        let  a: u64 = 1024;
        let b: i8 = -7;
        let c : usize = 800;
        let d = -64;
    }

    #[test]
    fn test_numeric_representations_vars() {
        let a = 123_456;
        let b = 0xf2; //  hexadecimal
        let c = 0o71;	// octal
        let d = 0b1110_0001;	// binary
        let c = b'C';	// byte
    }

    #[test]
    fn test_u8_overflow() {
        #[allow(overflowing_literals)]
        let a:i8 = 0xf_f;
        assert_eq!(a, -1);
    }

    #[test]
    fn test_float_declaration() {
        let b: f32 = 2.95;
        let a = 2.95;
    }
}