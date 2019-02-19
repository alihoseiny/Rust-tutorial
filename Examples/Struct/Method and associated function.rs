#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
    courses: [Course; 3]
}

#[derive(Debug)]
struct Course {
    name: String,
    passed: bool
}

impl Student {
    fn check_graduation(self) -> bool {
        for course in self.courses.iter() {
            if !course.passed {
                return false;
            }
        }
        return true;
    }

    fn construct(name: String, id: u32, courses: [Course; 3]) -> Student{
        Student {
            name,
            id,
            courses,
        }
    }
}

impl Course {
    fn print(&self) {
        println!("Course in the print method: {:#?}", self);
    }
}

impl Course {
    fn pass(&mut self) {
        self.passed = true;
    }
}

fn main() {

    let course1 = Course {
        name: String::from("مهندسی نرم افزار"),
        passed: true
    };

    let course2 = Course {
        name: String::from("طراحی پایگاه داده"),
        passed: true
    };

    let course3 = Course {
        name: String::from("روش های رسمی در مهندسی نرم افزار"),
        passed: true
    };

    let mut asghar = Student::construct(String::from("اصغر اکبرآبادی اصل"), 96532147, [course1, course2, course3]);
    println!("Asghar revived with an associated function: {:#?}", asghar);
    println!("Checking asghar graduation result is: {}", asghar.check_graduation());

    let mut course_instance = Course {
        name: String::from("یک اسم الکی"),
        passed: false
    };
    course_instance.print();
    course_instance.pass();
    course_instance.print();
}
