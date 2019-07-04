///
/// ```compile_fail
/// fn main() {
///    let local_variable = "این یک متغیر محلی است و خارج از تابع main قابل دسترس نیست.";
///    println!("{}", local_variable);
///    another_function();
///}
///fn another_function() {
///    println!("{}", local_variable);   // Error!
///}
/// ```
///
fn test1(){}

///
/// ```compile_fail
/// let a = String::from("hello");
///    let b = a;
///    println!("a: {}", a);
///    println!("b: {}", b);
/// ```
fn test2(){}

///
/// ```compile_fail
///     fn main() {
///        let a = String::from("hello");
///        i_am_owner(a);
///        println!("a in main function: {}", a);
///    }
///    fn i_am_owner(input: String) {
///        println!("The input value is: {}", input);
///    }
/// ```

fn test3(){}