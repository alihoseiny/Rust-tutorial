#[cfg(test)]
mod test11 {

    #[derive(Debug)]
    struct Course {
        name: String,
        passed: bool
    }

    #[derive(Debug)]
    struct Student {
        name: String,
        id: u32,
        courses: [Course; 3]
    }

    impl Student {
        fn check_graduation(&self) -> bool {
            for course in self.courses.iter() {
                if !course.passed {
                    return false;
                }
            }
            return true;
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

    impl Student {
        fn construct(name: String, id: u32, courses: [Course; 3]) -> Student{
            Student {
            name,
            id,
            courses,
            }
        }
    }

    #[test]
    fn test_method_with_getting_ownership() {
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
        let asghar = Student {
            name: String::from("اصغر اکبرآبادی اصل"),
            id: 96532147,
            courses: [course1, course2, course3]
        };
        assert_eq!(asghar.check_graduation(), true);
    }

    #[test]
    fn test_method_without_getting_ownership() {
        let course_instance = Course {
            name: String::from("یک اسم الکی"),
            passed: false
        };
        course_instance.print();
        println!("Course in the main function: {:#?}", course_instance);    // If ownership has been taken throws an error
    }

    #[test]
    fn test_changing_value_using_method() {
        let mut course_instance = Course {
            name: String::from("یک اسم الکی"),
            passed: false
        };
        assert_eq!(course_instance.passed, false);
        course_instance.pass();
        assert_eq!(course_instance.passed, true);
    }

    #[test]
    fn test_struct_construction_using_associated_function() {
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
        let asghar = Student::construct(String::from("اصغر اکبرآبادی اصل"), 96532147, [course1, course2, course3]);
        assert_eq!(asghar.name, "اصغر اکبرآبادی اصل");
        assert_eq!(asghar.id, 96532147);
    }
}