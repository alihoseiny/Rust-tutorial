///
/// ```compile_fail
///     fn main() {
///        let mut my_array = [-5, -3, -10, 0, 1, 8, 9];
///        let not_negative_item = first_not_negative_element(&my_array);
///        if not_negative_item.len() == 1 {
///            println!("First not negative element in the my_array is: {:?}", not_negative_item);
///        } else {
///            println!("All elements of my_array are negative.");
///        }
///        my_array = [0i32; 7];
///        println!("Incorrect not negative value: {:?}", my_array);
///    }
///    fn first_not_negative_element(array: &[i32; 7]) -> &[i32] {
///        for (index, &item) in array.iter().enumerate() {
///            if item > -1 {
///                return &array[index..index + 1];
///            }
///        }
///        return &array[0..array.len()];
///    }
/// ```
fn test1(){}