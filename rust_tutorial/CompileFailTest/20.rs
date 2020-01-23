///```compile_fail
/// fn main()
/// {
///    let a;
///    {
///        let b = 10;
///        a = &b;
///    }
///    println!("a = {}", a);
///}
///```
fn test1(){}

///```compile_fail
/// #[derive(Debug)]
///  struct  MyStruct {
///    name: &str,
///    age: u8
///}
/// fn main() {
///    let name = "Asghar";
///    let person = MyStruct {
///        name: name,
///        age: 10
///    };
///    println!("{:?}", person);
///}
/// ```
fn test2(){}