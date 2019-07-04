///
/// ```compile_fail
///     struct Student {
///        name: String,
///        id: u32,
///        graduated: bool
///    }
///    fn main() {
///        let asghar = Student {
///            id: 963258741,
///            name: String::from("اصغر اکبرآبادی اصل"),
///            graduated: false
///        };
///        println!("{}", asghar);
///    }
/// ```
fn test1(){}

///
/// ```compile_fail
/// #[derive(Debug)]
///struct Student {
///    name: String,
///    id: u32,
///    graduated: bool
///}
/// fn main() {
///    let asghar = Student {
///        id: 963258741,
///        name: String::from("اصغر اکبرآبادی اصل"),
///        graduated: false
///    };
///    asghar.name = String::from("اکبر اصغر زاده");
///    println!("Student name: {}", asghar.name);
///}
/// ```
fn test2(){}


