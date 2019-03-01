#[derive(Debug)]
struct Course {
    name: String,
    passed: bool,
    teacher: Teacher
}

#[derive(Debug)]
struct Teacher {
    name: String,
    course: Course
}


fn main() {
    let course: Course;
    course = Course {
        name: String::from("درس۱"),
        passed: false,
        teacher: Teacher {
            name: String::from("عین الله"),
            course
        }
    };
}

