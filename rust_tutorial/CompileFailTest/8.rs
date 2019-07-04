///
/// ```compile_fail
///     fn main() {
///        let a = String::from("hello");
///        i_am_owner(&a);
///        println!("a in main function: {}", a);
///    }
///    fn modifier(reference: &String) {
///        reference.push_str(" a new string to push to the old one");
///    }
/// ```
fn test1(){}

///
/// ```compile_fail
///     fn main() {
///        let mut a = String::from("hello");
///        modifier(&a);
///        println!("a in main function: {}", a);
///    }
///    fn modifier(reference: &String) {
///        reference.push_str(" a new string to push to the old one");
///    }
/// ```
fn test2(){}

///
/// ```compile_fail
///     fn main() {
///        let a = String::from("hello");
///        modifier(&mut a);
///        println!("a in main function: {}", a);
///    }
///    fn modifier(reference: &mut String) {
///        reference.push_str(" a new string to push to the old one");
///    }
/// ```
fn test3(){}

///
/// ```compile_fail
/// fn main() {
///    let mut a = String::from("hello");
///    let reference1 = &a;
///    let reference2 = &a;
///    let reference3 = &mut a;
///    ali(reference1);
///    mohammad(reference3);
///    hossein(reference2);
///    println!("a in main function: {}", a);
///}
///fn ali(original_text: &String) {
///    println!("Ali says: {}", original_text);
///}
///fn hossein(text: &String) {
///    println!("{} hossein", text);
///}
///fn mohammad(original_input: &mut String) {
///    original_input.push_str("!");
///}
/// ```
fn test4(){}

///
/// ```compile_fail
///     fn main() {
///       let b = dangle_generator();
///        println!("a in main function: {}", b);
///    }
///    fn dangle_generator() -> &String {
///        let a = String::from("hello");
///        &a
///    }
/// ```
fn test5(){}