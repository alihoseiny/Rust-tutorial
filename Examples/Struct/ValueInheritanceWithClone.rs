#[derive(Debug, Clone)]
struct Student {
    name: String,
    id: u32,
    courses: [Course; 3]
}

#[derive(Debug, Clone)]
struct Course {
    name: String,
    passed: bool,
}

fn main() {
    let courses = [Course {name: String::from("درس۱"), passed: false},
                   Course {name: String::from("درس۲"), passed: false},
                   Course {name: String::from("درس۳"), passed: false}];
    let student1 = Student {
        name: String::from("اصغر اکبرزاده اصل"),
        id: 97959493,
        courses
    };
    let student2 = Student {id: 98999694, ..student1.clone()};
    println!("student1: {:#?}", student1);
    println!("student2: {:#?}", student2);
}

