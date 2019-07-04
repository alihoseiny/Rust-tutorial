#[cfg(test)]
mod test7 {
    const GLOBAL_CONSTANT: &str = "test";

    fn i_am_owner(input: String) -> String {
        return input;
    }

    #[test]
    fn test_global_constant_access_in_function() {
        assert_eq!("test", GLOBAL_CONSTANT);
    }

    #[test]
    fn test_string_creation() {
        let my_string = String::from("یک رشته جدید.");
        assert_eq!(my_string, "یک رشته جدید.");
    }

    fn test_push_string() {
        let mut my_string = String::from("یک رشته جدید");
        assert_eq!(my_string, "یک رشته جدید");
        my_string.push_str(" که این متن به انتهایش اضافه شده است.");
        assert_eq!("یک رشته جدید که این متن به انتهایش اضافه شده است.", my_string);
    }

    #[test]
    fn test_ownership_move_and_back() {
        let mut a = String::from("hello");
        a = i_am_owner(a);
        assert_eq!(a, "hello");
    }
}