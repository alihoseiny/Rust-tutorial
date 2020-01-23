#[cfg(test)]
mod test20 {
    fn greatest_sum <'a>(array1: &'a [i32], array2: &'a [i32]) -> &'a [i32] {
        let mut sum1 = 0;
        let mut sum2 = 0;

        for i in array1 {
            sum1 += *i;
        }

        for j in array2 {
            sum2 += *j;
        }

        if sum1 > sum2 {
            return array1;
        }
        return array2;
    }

    #[derive(Debug)]
    struct  MyStruct<'a> {
        name: &'a str,
        age: u8
    }

    impl<'a> MyStruct<'a> {
        fn change_name(&mut self, new_name: &'a str) -> &str {
            let old_name = self.name;
            self.name = new_name;
            return old_name;
        }
    }

    #[derive(Debug)]
    enum Something <'a> {
        Some(&'a u8),
        Thing(u8)
    }

    fn call_me() {
        static mut NUMBER_OF_CALLS: u8 = 0;
        unsafe {
            NUMBER_OF_CALLS += 1;
            println!("number of calls: {}", NUMBER_OF_CALLS);
        }
    }

    #[test]
    fn test_function_lifetime() {
        let arr1 = [1, 2, 3];
        let arr2 = [1, 2, 3, 4];
        let greatest_array = greatest_sum(&arr1, &arr2);

        assert_eq!(&arr2, greatest_array);
    }

    #[test]
    fn test_struct_method_lifetime() {
        let name = "Asghar";
        let mut person = MyStruct {
            name,
            age: 10
        };

        assert_eq!(person.name, "Asghar");

        person.change_name("Akbar");

        assert_eq!(person.name, "Akbar");
    }

    #[test]
    fn test_enum_lifetime_syntax() {
        let number: u8 = 10;
        let a = Something::Some(&number);
    }

    #[test]
    fn test_mutable_static_syntax() {
        call_me();
    }
}