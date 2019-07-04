#[cfg(test)]
mod test5 {
    #[test]
    fn test_loop_syntax() {
        loop {
            break;
        }
    }

    #[test]
    fn test_while() {
        let mut a = 1;
        while a % 10 != 0 {
            a += 1;
        }
        assert_eq!(a, 10);
    }

    #[test]
    fn test_for() {
        let mut result_array = [0i32;9];
        let  expected_array: [i32;9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
        for counter in 2..11 {
            result_array[counter - 2] = counter as i32;
        }
        assert_eq!(expected_array, result_array);
    }

    #[test]
    fn test_array_iteration_by_for() {
        let my_array: [i32;5] = [1,2,3,4,5];
        let mut expected_value: i32 = 0;
        for counter in 0..5 {
            expected_value += 1;
            assert_eq!(my_array[counter], expected_value);
        }
    }

    #[test]
    fn test_array_iteration_by_for_using_iter() {
        let my_array: [i32;5] = [1,2,3,4,5];
        let mut expected_value: i32 = 0;
        for element in my_array.iter(){
            expected_value += 1;
            assert_eq!(*element, expected_value);   // using * because iter returns an reference
        }
    }
    
}