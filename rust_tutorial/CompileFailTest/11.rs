///
/// ```compile_fail
/// #[derive(Debug)]
///struct Student {
///    name: String,
///    id: u32,
///    courses: [Course; 3]
///}
///
///#[derive(Debug)]
///struct Course {
///    name: String,
///    passed: bool
///}
/// impl Student {
///    fn check_graduation(self) -> bool {
///        for course in self.courses.iter() {
///            if !course.passed {
///                return false;
///            }
///        }
///        return true;
///    }
///}
///
/// fn main() {
///    let course1 = Course {
///        name: String::from("مهندسی نرم افزار"),
///        passed: true
///    };
///    let course2 = Course {
///        name: String::from("طراحی پایگاه داده"),
///        passed: true
///    };
///    let course3 = Course {
///        name: String::from("روش های رسمی در مهندسی نرم افزار"),
///        passed: true
///    };
///    let asghar = Student {
///        name: String::from("اصغر اکبرآبادی اصل"),
///        id: 96532147,
///        courses: [course1, course2, course3]
///    };
///    println!("Checking asghar graduation result is: {}", asghar.check_graduation());
///    print!("Asghar: {:#?}", asghar);
///}
/// ```
fn test1(){}