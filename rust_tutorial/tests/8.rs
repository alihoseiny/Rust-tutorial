#[cfg(test)]
mod test8 {
    fn i_am_owner(input: &String) {
        println!("The input value is: {}", input);
    }

    fn i_am_owner_referenced(input: &String) {
        assert_eq!(*input, "hello");
    }

    fn modifier(reference: &mut String) {
        reference.push_str(" a new string to push to the old one");
    }

    fn ali(original_text: &String) {
        println!("Ali says: {}", original_text);
    }
    fn hossein(text: &String) {
        println!("{} hossein", text);
    }
    fn mohammad (original_input: &mut String) {
        original_input.push_str("!");
    }

    #[test]
    fn test_reference_passing() {
        let a = String::from("hello");
        i_am_owner(&a);
        assert_eq!(a, "hello");
    }

    #[test]
    fn test_dereferencing() {
        let a = String::from("hello");
        i_am_owner_referenced(&a);
    }

    #[test]
    fn test_borrowing() {
        let mut a = String::from("hello");
        modifier(&mut a);
        assert_eq!(a, "hello a new string to push to the old one");
    }

    #[test]
    fn test_multiple_references() {
        let mut a = String::from("hello");
        ali(&a);
        mohammad(&mut a);
        hossein(&a);
        assert_eq!(a, "hello!");
    }

}