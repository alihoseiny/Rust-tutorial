
//  خطوطی که این‌شکلی با دوتا / شروع می‌شوند کامنت هستند. یعنی توضیحاتی اضافی هستند و جزو کد محسوب نمی‌شوند
// چون خطوط به هم می‌ریزند از این به بعد کامنت‌هارا انگلیسی می‌نویسم. ولی تمام تلاشم را می‌کنم که تا حد امکان ساده باشند.

fn factorial_recursive(n: i32) -> i32{
    if n < 2 {  // 0! and 1! both are equal to 1 and factorial only defined on not negative numbers
        return 1;
    }
        n * factorial_recursive(n - 1)    // Like the arithmetic formula, n! = n * (n - 1 )!
}


fn factorial_for(n: i32) -> i32{
    let mut factorial_result = 1;      // Storing the result of the (n - 1)!
                                          // Because 0! and 1! are equal to 1, we chose the
                                          // 1 initial value for this variable.

    for number in 1..n + 1 { // for looping in numbers from 1 to n, we should create a range from 1 to n + 1
        factorial_result*= number;   // Creating the 1 * 2 * ... * n
    }
    return factorial_result;
}





fn main() {

    println!("factorial_recursive function results: ");
    // \t is a tab character. Don't worry about it.
    println!("\t{}! is equal to: {}", 0, factorial_recursive(0));
    println!("\t{}! is equal to: {}", 1, factorial_recursive(1));
    println!("\t{}! is equal to: {}", 5, factorial_recursive(5));
    println!("\t{}! is equal to: {}", 7, factorial_recursive(7));
    println!("\t{}! is equal to: {}", 10, factorial_recursive(10));

    println!("\n\n\n\n"); // Just adding 4 new lines to separating results printed in the terminal.

    println!("factorial_for function results: ");
    // \t is a tab character. Don't worry about it.
    println!("\t{}! is equal to: {}", 0, factorial_for(0));
    println!("\t{}! is equal to: {}", 1, factorial_for(1));
    println!("\t{}! is equal to: {}", 5, factorial_for(5));
    println!("\t{}! is equal to: {}", 7, factorial_for(7));
    println!("\t{}! is equal to: {}", 10, factorial_for(10));
}
