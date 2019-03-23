// خطوطی که به این شکل با دو "/" شروع می‌شوند کامنت هستند. یعنی توضیحاتی اضافی هستند و جزو کد محسوب نمی‌شوند
// چون خطوط به هم می‌ریزند از این به بعد کامنت‌ها را انگلیسی می‌نویسم. ولی تمام تلاشم را می‌کنم که تا حد امکان ساده باشند

fn factorial_recursive(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    
    // Like the arithmetic formula:
    // n! = n * (n - 1) * (n - 2) ... * 2 * 1
    n * factorial_recursive(n - 1)
}

fn factorial_with_for(n: i32) -> i32 {
    // Storing the result of (n - 1)
    let mut result = 1;

    // For looping in numbers from 1 to n, we should create a range from 1 to n + 1
    for number in 1..n + 1 {
        // Creating the: 1 * 2 * ... * n
        result *= number;
    }

    result
}

fn main() {

    // "\t" is a tab
    println!("factorial_recursive function results:");
    println!("{}!\tis equal to:\t{}", 0, factorial_recursive(0));
    println!("{}!\tis equal to:\t{}", 1, factorial_recursive(1));
    println!("{}!\tis equal to:\t{}", 5, factorial_recursive(5));
    println!("{}!\tis equal to:\t{}", 7, factorial_recursive(7));
    println!("{}!\tis equal to:\t{}", 10, factorial_recursive(10));

    print!("\n"); // New line

    println!("factorial_with_for function results:");
    println!("{}!\tis equal to:\t{}", 0, factorial_with_for(0));
    println!("{}!\tis equal to:\t{}", 1, factorial_with_for(1));
    println!("{}!\tis equal to:\t{}", 5, factorial_with_for(5));
    println!("{}!\tis equal to:\t{}", 7, factorial_with_for(7));
    println!("{}!\tis equal to:\t{}", 10, factorial_with_for(10));

}
