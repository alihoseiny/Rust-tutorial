///```compile_fail
///fn test() {
///    let x = 10;
///    x = 5;
///}
///```
fn test1() {

}

///
/// This test does not check compile error when you define variable in the global scope, but it should do!
/// ```compile_fail
/// const BUFFER_SIZE: i32 = 10;
//let x = 10;
//
//fn main() {
//   println!("سلام دنیا!");
//}```
///
fn test2(){}



///
/// This error is not because of assigning a function to a variable, is because defining a constant
/// if another scope rather than global. But it should test the first case
/// ```compile_fail
/// const BUFFER_SIZE: i32 = function();
/// ```
fn test3(){}

