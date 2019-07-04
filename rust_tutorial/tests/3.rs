#[cfg(test)]
mod test3 {
    #[test]
    fn test_array_declaration() {
            let months = ["فروردین", "اردیبهشت", "خرداد",
                          "تیر", "مرداد", "شهریور",
                          "مهر", "آبان", "آذر",
                          "دی", "بهمن", "اسفند"];
    }

    #[test]
    fn test_array_access_by_index() {
            let months = ["فروردین", "اردیبهشت", "خرداد",
                          "تیر", "مرداد", "شهریور",
                          "مهر", "آبان", "آذر",
                          "دی", "بهمن", "اسفند"];
    assert_eq!(months[0], "فروردین");
    assert_eq!(months[11], "اسفند");
    }

    fn test_array_element_modification_by_index() {
            let mut months = ["فروردین", "اردیبهشت", "خرداد",
                          "تیر", "مرداد", "شهریور",
                          "مهر", "آبان", "آذر",
                          "دی", "بهمن", "اسفند"];
    assert_eq!(months[6], "مهر");
    months[6] = "محمّدرضا علی حسینی";
    assert_eq!(months[6], "محمّدرضا علی حسینی");
    }
    
    #[test]
    fn test_array_shorthand() {
        let array = [0i32;7];
        assert_eq!(array, [0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_array_printing() {
        println!("my array is: {:?}", [10i8;10]);
    }

    #[test]
    fn test_tuple_declaration() {
        let tup0: (i32, char, bool, f64);
        let tup1 = (1, true, "سلام", 9.99);
        tup0 = (33, 'G', false, 9.87);
    }

    #[test]
    fn test_tuple_access_by_index() {
        let tup = (1, true, "سلام", 9.99);
        assert_eq!(tup.2, "سلام");
    }

    #[test]
    fn test_tuple_extract_by_pattern_matching() {
        let tup = (1, true, "سلام", 9.99);
        let (x, y, v, z) = tup;
        assert_eq!(x, 1);
        assert_eq!(y, true);
        assert_eq!(v, "سلام");
        assert_eq!(z, 9.99);
    }
}
