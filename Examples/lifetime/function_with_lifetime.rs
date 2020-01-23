fn greatest_sum <'a>(array1: &'a [i32], array2: &'a [i32]) -> &'a [i32] {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in array1 {
        sum1 += *i;
    }

    for j in array2 {
        sum2 += *j;
    }

    if sum1 < sum2 {
        return array1;
    }
    return array2;
}

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3, 4];

    let greatest_array = greatest_sum(&arr1, &arr2);
    println!("greatest array: {:?}", greatest_array);
}
