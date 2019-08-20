///
/// ```compile_fail
/// trait Brush {
///    fn draw(&self);
///    fn change_colour(&self);
///}
///
///trait Screen {
///    fn draw(&self);
///    fn turnoff(&self);
///}
///
///struct Something();
///
///impl Brush for Something {
///    fn draw(&self) {
///        println!("Draw a line on the paper.");
///    }
///
///    fn change_colour(&self) {
///        println!("Brush colour changed.");
///    }
///}
///
///impl Screen for Something {
///    fn draw(&self) {
///        println!("Draw a line on the screen");
///    }
///
///    fn turnoff(&self) {
///        println!("Screen turned off");
///    }
///}
/// fn main() {
///    let something = Something{};
///    something.draw();
///}
/// ```
fn test1(){}

///
/// ```compile_fail
/// let x = -10;
///x.abs(x);
/// ```
fn test2(){}