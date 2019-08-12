///
/// ```compile_fail
/// trait Animal {
///    fn eat(&self);
///}
///
///trait Fly: Animal {
///    fn fly(&self);
///    fn land(&self) {
///        println!("Flyable Object now landing.");
///    }
///}
///struct Kaftar ();
///
///impl Fly for Kaftar {
///    fn fly(&self) {
///        println!("Kafter The Kakol Be Sar is flying");
///   }
///}
/// fn main() {
///    let kakol_be_sar = Kaftar{};
///    kakol_be_sar.fly();
///    kakol_be_sar.eat();
///}
/// ```
fn test1(){}