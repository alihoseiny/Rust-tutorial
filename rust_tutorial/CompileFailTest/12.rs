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
/// fn main() {
///
///    let courses = [Course {name: String::from("درس۱"), passed: false},
///        Course {name: String::from("درس۲"), passed: false},
///        Course {name: String::from("درس۳"), passed: false}];
///
///    let student1 = Student {
///        name: String::from("اصغر اکبرزاده اصل"),
///        id: 9796959493,
///        courses
///    };
///
///    let student2 = Student {id: 9899969594, ..student1.clone()};
///
///    println!("student1: {:#?}", student1);
///    println!("student2: {:#?}", student2);
///}
/// ```
fn test1(){}

///
/// ```compile_fail
/// #[derive(Debug, Clone)]
///struct Course {
///    name: String,
///    passed: bool,
///    teacher: Teacher
///}
///
///#[derive(Debug)]
///struct Teacher {
///    name: String,
///    course: Course
///}
/// fn main() {
///    let course: Course;
///     course = Course {
///        name: String::from("درس۱"),
///        passed: false,
///        teacher: Teacher {
///            name: Student::from("عین الله"),
///            course
///        }
///    };
///}
///```
fn test2(){}