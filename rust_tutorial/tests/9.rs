#[cfg(test)]
mod test9 {
    fn first_not_negative_element(array: [i32; 7]) -> i32 {
        for (index, &item) in array.iter().enumerate() {
            if item > -1 {
                return item;
            }
        }
        return -1;
    }

    fn first_not_negative_element_sliced(array: &[i32; 7]) -> &[i32] {
        for (index, &item) in array.iter().enumerate() {
            if item > -1 {
                return &array[index..index + 1];
            }
        }
        return &array[0..array.len()];
    }

    #[test]
    fn test_first_not_negative_element() {
        let my_array = [-5, -3, -10, 0, 1, 8, 9];
        let not_negative_item = first_not_negative_element(my_array);
        assert_eq!(not_negative_item, 0);
    }

    #[test]
    fn test_first_not_negative_element_sliced() {
        let mut my_array = [-5, -3, -10, 0, 1, 8, 9];
        let not_negative_item = first_not_negative_element_sliced(&my_array);
        assert_eq!([0], not_negative_item);
    }

    #[test]
    fn test_first_not_negative_element_scoped() {
        let mut my_array = [-5, -3, -10, 0, 1, 8, 9];
    {   // New scope for slicing my_array
        let not_negative_item = first_not_negative_element_sliced(&my_array);
        assert_eq!(not_negative_item, [0]);
    }   // End of the scope
    my_array = [7i32; 7];
    assert_eq!(first_not_negative_element_sliced(&my_array), [7]);
    }

    #[test]
    fn test_first_not_negative_element_copy() {
        let my_array = [-5, -3, -10, 0, 1, 8, 9];
        let not_negative_item = first_not_negative_element_sliced(&my_array);
        assert_eq!([0], not_negative_item);
        let mut my_second_array = my_array; // copying my_array to new variable
        my_second_array[0] = 100;
        assert_eq!([100], first_not_negative_element_sliced(&my_second_array));
    }

}