#[cfg(test)]
mod test6 {

    fn duplicator(input_number : i32) -> i32 {
        let result = input_number * 2;
        return result;
    }

    fn duplicator_without_return(input_number : i32) -> i32 {
       input_number * 2
    }

    fn recursive_function(mut input_number: i32) {
        if input_number < 1 {
            return;
        }
        println!("{}", input_number);
        input_number -= 1;
        recursive_function(input_number);
    }

    #[test]
    fn test_duplicator_function() {
        assert_eq!(4, duplicator(2));
    }

    #[test]
    fn test_duplicator_without_return() {
        assert_eq!(4, duplicator_without_return(2));
    }

    #[test]
    /// Only tests syntax, not functionality
    fn test_recursive_function_syntax() {
        recursive_function(10);
    }
}